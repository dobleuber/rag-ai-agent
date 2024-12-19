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

async fn prompt(query: &str) -> Result<String> {
    let qdrant_client = create_qdrant_client()?;
    let state = state::AppState {
        agent: MyAgent::new(qdrant_client),
    };

    state.agent.init().await?;

    // TODO: get file from user
    // let file = file::File::new()?;

    let response = state.agent.prompt(query.to_string()).await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    println!("Do you have any questions?");
    let user_input = io_utils::get_user_input();
    println!("You said: {}", user_input);

    let response = prompt(&user_input).await?;

    println!("{}", response);
    Ok(())
}
