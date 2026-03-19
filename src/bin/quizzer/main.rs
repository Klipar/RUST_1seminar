//! Quizzer binary - main entry point

use quizzer::{Question, import_from_json};

use clap::Parser;
use std::process;
use std::io;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct WorkingMod {
    state: String,
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read error");
    input.trim().to_owned()
}

fn main() {
    let work_mod = match WorkingMod::try_parse() {
        Ok(work_mod) => {
            if work_mod.state == "question-entering" || work_mod.state == "quiz"{
                work_mod
            } else {
                eprintln!("Unknown working mode, please try again");
                process::exit(1);
            }
        },
        Err(e) => {
            if e.to_string().contains("unexpected argument") {
                eprintln!("Unexpected argument");
            } else {
                eprintln!("{}", e);
            }
            process::exit(1);
        }
    };

    if work_mod.state == "question-entering" {
        loop {
            println!("Enter new question or \"exit\" to exit the program");

            let question_text = read_input();

            if question_text == "exit"{
                process::exit(0);
            }

            println!("Enter correct answer:");
            let mut answers = vec![String::new(); 4];
            answers[0] = read_input(); // correct answer

            for i in 1..4 {
                println!("Enter incorrect answer:");
                answers[i] = read_input();
            }

            let question = Question {
                question: question_text,
                options: answers.try_into().expect("You need exactly 4 answers"),
            };

            question.append_to_json();

            println!("The new entry has been added:");
        }
    } else {
        let question_set =  import_from_json();
        let mut true_answers = 0;

        for q in &question_set{
            println!("\n{}\nOptions:", q.question);

            let mut indices: Vec<usize> = (0..q.options.len()).collect();
            indices.shuffle(&mut thread_rng());

            for i in indices {
                println!("{}", q.options[i]);
            }

            let user_answer = read_input();

            if user_answer == q.options[0]{
                true_answers+=1;
            }
        }

        println!("\nYour score: {}/{}", true_answers, question_set.len());
    }
}
