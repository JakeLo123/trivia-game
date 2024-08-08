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
pub struct OpenTResponse {
    pub results: Vec<OpenTResult>,
}
