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
