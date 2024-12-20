use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        CreateEmbeddingRequest, EmbeddingInput,
    },
    Client as OpenAIClient, Embeddings,
};
use qdrant_client::{
    prelude::Payload,
    qdrant::{
        with_payload_selector::SelectorOptions, CreateCollectionBuilder, Distance, PointStruct,
        SearchPoints, UpsertPointsBuilder, VectorParamsBuilder, WithPayloadSelector,
    },
    Qdrant,
};

use anyhow::{anyhow, Result};

use crate::file::File;

static COLLECTION: &str = "csv-files";
static GPT_MODEL: &str = "gpt-4o-mini";
static EMBEDDING_MODEL: &str = "text-embedding-3-small";
static EMBEDDING_DIMENSIONS: u32 = 512;

static SYSTEM_PROMPT: &str = "You are a world-class data analyst, specialising in analysing comma-delimited CSV files.
      Your job is to analyse some CSV snippets and determine what the results are for the question that the user is asking.
      You should aim to be concise. If you don't know something, don't make it up but say 'I don't know.'.";

pub struct MyAgent {
    openai_client: OpenAIClient<OpenAIConfig>,
    qdrant_client: Qdrant,
}

pub trait Agent {
    fn new(qdrant_client: Qdrant) -> Self;
    async fn init(&self) -> Result<()>;
    async fn get_embedding(&self, file: File) -> Result<()>;
    async fn search_documents(&self, prompt: String) -> Result<String>;
    async fn prompt(&self, prompt: String) -> Result<String>;
}

impl Agent for MyAgent {
    fn new(qdrant_client: Qdrant) -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
        let config = OpenAIConfig::new().with_api_key(api_key);
        let openai_client = OpenAIClient::with_config(config);

        Self {
            openai_client,
            qdrant_client,
        }
    }

    async fn init(&self) -> Result<()> {
        self.qdrant_client
            .create_collection(CreateCollectionBuilder::new(COLLECTION).vectors_config(
                VectorParamsBuilder::new(EMBEDDING_DIMENSIONS.into(), Distance::Cosine),
            ))
            .await?;

        Ok(())
    }

    async fn get_embedding(&self, file: File) -> Result<()> {
        if file.rows.is_empty() {
            return Err(anyhow!("There are no rows in the file"));
        }

        let request = CreateEmbeddingRequest {
            model: EMBEDDING_MODEL.to_string(),
            input: EmbeddingInput::StringArray(file.rows.clone()),
            user: None,
            dimensions: Some(EMBEDDING_DIMENSIONS),
            ..Default::default()
        };

        let embeddings_result = Embeddings::new(&self.openai_client).create(request).await?;

        for embedding in embeddings_result.data {
            let payload: Payload = serde_json::json!({
              "id": file.path.clone(),
              "content": file.contents,
              "rows": file.rows,
            })
            .try_into()
            .unwrap();

            println!("embedded: {:#?}", file.path);

            let vec = embedding.embedding;
            let points = vec![PointStruct::new(
                uuid::Uuid::new_v4().to_string(),
                vec,
                payload,
            )];

            self.qdrant_client
                .upsert_points(UpsertPointsBuilder::new(COLLECTION, points))
                .await?;
        }

        Ok(())
    }

    async fn search_documents(&self, prompt: String) -> Result<String> {
        let request = CreateEmbeddingRequest {
            model: EMBEDDING_MODEL.to_string(),
            input: EmbeddingInput::String(prompt),
            user: None,
            dimensions: Some(EMBEDDING_DIMENSIONS),
            ..Default::default()
        };

        let embeddings_result = Embeddings::new(&self.openai_client).create(request).await?;

        let embedding = &embeddings_result.data.first().unwrap().embedding;

        let payload_selector = WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(true)),
        };

        let search_points = SearchPoints {
            collection_name: COLLECTION.to_string(),
            vector: embedding.to_vec(),
            limit: 1,
            with_payload: Some(payload_selector),
            ..Default::default()
        };

        let search_result = self.qdrant_client.search_points(search_points).await?;
        let result = search_result.result.into_iter().next();

        match result {
            Some(point) => Ok(point.payload.get("content").unwrap().to_string()),
            None => Err(anyhow!("There were no results that matched :(")),
        }
    }

    async fn prompt(&self, prompt: String) -> Result<String> {
        let context = self.search_documents(prompt.clone()).await?;
        let input = format!(
            "{prompt}
          
          Context:
          {context}",
        );

        let res = self
            .openai_client
            .chat()
            .create(
                CreateChatCompletionRequestArgs::default()
                    .model(GPT_MODEL)
                    .messages(vec![
                        ChatCompletionRequestMessage::System(
                            ChatCompletionRequestSystemMessageArgs::default()
                                .content(SYSTEM_PROMPT)
                                .build()?,
                        ),
                        ChatCompletionRequestMessage::User(
                            ChatCompletionRequestUserMessageArgs::default()
                                .content(input)
                                .build()?,
                        ),
                    ])
                    .build()?,
            )
            .await
            .map(|res| {
                res.choices
                    .first()
                    .unwrap()
                    .message
                    .content
                    .clone()
                    .unwrap()
            })?;

        Ok(res)
    }
}
