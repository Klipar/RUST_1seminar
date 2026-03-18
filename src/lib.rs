//! Quizzer library

#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub options: [String; 4],
}

/// Returns a greeting message
pub fn get_greeting() -> String {
    String::from("Hello from the quizzer library!")
}

impl Question {
    pub fn save(&self) {
        println!("Sawed question: {}", self.question);
    }
}

pub fn get_question() -> Vec<Question> {
    let mut questions = Vec::new();
    questions.push(Question {
        question: "2 + 2 = ?".to_string(),
        options: [
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
        ],
    });

    questions.push(Question {
        question: "Столиця Франції?".to_string(),
        options: [
            "Лондон".to_string(),
            "Париж".to_string(),
            "Берлін".to_string(),
            "Мадрид".to_string(),
        ],
    });
    questions.push(Question {
        question: "Otakza od zaka fresko?".to_string(),
        options: ["Tak".to_string(), "niiii".to_string(), "may be".to_string(), "xto ja".to_string()]
    });
    questions
}
