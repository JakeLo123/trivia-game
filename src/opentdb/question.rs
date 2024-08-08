use super::opentdb_client::OpenTResult;
use rand::Rng;

pub struct Question {
    pub description: String,
    pub answers: Vec<String>,
    pub correct_answer: String,
}

impl Question {
    pub fn from_open_t_result(result: OpenTResult) -> Self {
        let mut all_answers: Vec<String> = result.incorrect_answers;
        all_answers.push(result.correct_answer.clone());
        let mut rng = rand::thread_rng();
        for i in (1..all_answers.len()).rev() {
            let j = rng.gen_range(0..=i);
            all_answers.swap(i, j);
        }
        return Question {
            description: result.question,
            answers: all_answers,
            correct_answer: result.correct_answer,
        };
    }
}
