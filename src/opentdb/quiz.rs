use super::{opentdb_client::OpenTResponse, question::Question};
use console::style;
use dialoguer::Select;
use html_escape::decode_html_entities;

pub struct Quiz {
    pub questions: Vec<Question>,
    pub score: usize,
}

impl Quiz {
    pub fn from_open_t_response(response: OpenTResponse) -> Quiz {
        Quiz {
            questions: response
                .results
                .into_iter()
                .map(|result| Question::from_open_t_result(result))
                .collect(),
            score: 0,
        }
    }

    pub fn begin(mut self) {
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

        println!("You got {} out of 5 correct.", style(self.score).bold());
        println!("\n");
    }
}
