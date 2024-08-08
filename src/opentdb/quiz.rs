use super::{
    opentdb_client::{OpenTCategory, OpenTClient},
    question::Question,
};
use console::style;
use dialoguer::Select;
use html_escape::decode_html_entities;

pub struct Quiz {
    pub questions: Vec<Question>,
    pub score: usize,
}

impl Quiz {
    pub async fn new() -> Quiz {
        let amount = Quiz::get_questions_amount();
        let category = Quiz::get_category().await;
        let question_type = Quiz::get_question_type();
        let difficulty_level = Quiz::get_difficulty_level();

        let res =
            OpenTClient::fetch_questions(amount, category, question_type, difficulty_level).await;
        let quiz = match res {
            Ok(open_t_json) => Quiz {
                questions: open_t_json
                    .results
                    .into_iter()
                    .map(|result| Question::from_open_t_result(result))
                    .collect(),
                score: 0,
            },
            Err(_) => {
                std::process::exit(1);
            }
        };

        return quiz;
    }
}

impl Quiz {
    fn get_questions_amount() -> usize {
        return Select::new()
            .with_prompt("Number of questions")
            .items(&vec!["1", "2", "3", "4", "5"])
            .interact()
            .unwrap()
            + 1; // zero-indexed
    }

    fn get_question_type() -> &'static str {
        let selection = Select::new()
            .with_prompt("Type of questions:")
            .items(&vec!["Any", "Multiple Choice", "True/False"])
            .interact()
            .unwrap();
        return match selection {
            1 => "multiple",
            2 => "boolean",
            _ => "",
        };
    }

    fn get_difficulty_level() -> &'static str {
        let selection = Select::new()
            .with_prompt("Difficulty level:")
            .items(&vec!["Any", "Easy", "Medium", "Hard"])
            .interact()
            .unwrap();
        return match selection {
            1 => "easy",
            2 => "medium",
            3 => "hard",
            _ => "",
        };
    }

    async fn get_category() -> OpenTCategory {
        let result = OpenTClient::fetch_categories().await;
        let mut categories = match result {
            Ok(categories) => categories,
            Err(_) => {
                std::process::exit(1);
            }
        };
        let items: Vec<String> = categories.clone().into_iter().map(|c| c.name).collect();
        let selection = Select::new()
            .with_prompt("Category:")
            .items(&items)
            .interact()
            .unwrap();
        return categories.remove(selection);
    }
}

impl Quiz {
    pub fn begin(mut self) {
        let questions_count = self.questions.len();

        for question in self.questions {
            println!("\n");

            let selection = Select::new()
                .with_prompt(decode_html_entities(&question.description))
                .items(&question.answers)
                .interact()
                .unwrap();

            if question.answers[selection] == question.correct_answer {
                println!("{}", style("DING DING DING!").bold().green());
                self.score += 1;
            } else {
                println!("{}", style("WRONG!").bold().red());
                println!(
                    "The correct answer was {}",
                    style(question.correct_answer).yellow()
                );
            }
            println!("\n");
        }

        println!(
            "You got {} out of {} correct.",
            style(self.score).bold(),
            style(questions_count).bold()
        );
        println!("\n");
    }
}
