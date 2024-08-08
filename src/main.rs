mod opentdb;

use opentdb::quiz::Quiz;
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let quiz = Quiz::new().await;
    quiz.begin();
    Ok(())
}
