use anyhow::Result;
use qdrant_client::Qdrant;

mod agents;
mod file;
mod io_utils;
mod state;

use agents::{Agent, MyAgent};

fn create_qdrant_client() -> Result<Qdrant> {
    let api_key = std::env::var("QDRANT_API_KEY").expect("QDRANT_API_KEY is not set");
    let url = std::env::var("QDRANT_URL").expect("QDRANT_URL is not set");
    let client = Qdrant::from_url(&url).api_key(api_key).build()?;

    Ok(client)
}

async fn init() -> Result<state::AppState> {
    let qdrant_client = create_qdrant_client()?;
    let state = state::AppState {
        agent: MyAgent::new(qdrant_client),
    };

    // state.agent.init().await?;

    // TODO: get file from user
    // let file = file::File::new("./titanic.csv".into())?;

    // state.agent.get_embedding(file).await?;

    Ok(state)
}

async fn prompt(state: &state::AppState, query: &str) -> Result<String> {
    let response = state.agent.prompt(query.to_string()).await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> ! {
    dotenv::dotenv().ok();

    let state = init().await.expect("Failed to initialize state");

    println!("Do you have any questions?");

    loop {
        let user_input = io_utils::get_user_input();
        let response = prompt(&state, &user_input)
            .await
            .expect("Failed to get response");

        println!("{}", response);

        println!("Do you have any further questions?");
    }
}
