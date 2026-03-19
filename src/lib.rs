//! Quizzer library
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: [String; 4],
}

impl Question {
    pub fn append_to_json(self) {
        let mut questions: Vec<Question> = match fs::read_to_string("quiz.json") {
            Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };
        questions.push(self);
        let json = serde_json::to_string(&questions).unwrap();
        fs::write("quiz.json", json).unwrap();
    }
}
pub fn import_from_json() -> Vec<Question> {
    match fs::read_to_string("quiz.json") {
        Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}
