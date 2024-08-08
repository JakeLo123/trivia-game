mod opentdb;

use crate::opentdb::opentdb_client::OpenTResponse;
use opentdb::quiz::Quiz;
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url: &str = "https://opentdb.com/api.php?amount=5&type=multiple";

    let response = reqwest::get(url).await?;
    let open_t_response = response.json::<OpenTResponse>().await?;
    let quiz = Quiz::from_open_t_response(open_t_response);

    quiz.begin();

    Ok(())
}
