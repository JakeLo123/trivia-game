// use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenTResult {
    pub difficulty: String,
    pub category: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

#[derive(Deserialize)]
pub struct OpenTJson {
    pub results: Vec<OpenTResult>,
}

#[derive(Deserialize, Clone)]
pub struct OpenTCategory {
    pub id: usize,
    pub name: String,
}

#[derive(Deserialize)]
struct OpenTCategoriesResponse {
    pub trivia_categories: Vec<OpenTCategory>,
}

pub struct OpenTClient {}

impl OpenTClient {
    pub async fn fetch_questions(
        amount: usize,
        category: OpenTCategory,
        question_type: &'static str,
        difficulty: &'static str,
    ) -> Result<OpenTJson, reqwest::Error> {
        let url = format!(
            "https://opentdb.com/api.php?amount={}&category={}&type={}&difficulty={}",
            amount, category.id, question_type, difficulty
        );
        let response = reqwest::get(url).await?;
        let body = response.json::<OpenTJson>().await?;
        return Ok(body);
    }

    pub async fn fetch_categories() -> Result<Vec<OpenTCategory>, reqwest::Error> {
        let response = reqwest::get("https://opentdb.com/api_category.php").await?;
        let body = response.json::<OpenTCategoriesResponse>().await?;
        return Ok(body.trivia_categories);
    }
}
