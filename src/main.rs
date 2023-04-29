// Crates
use rust_summarization::summarize_text;

pub fn main() {
    let text: &str = "Hello World! My name is John Doe and I live in the United States of America. I am a software engineer who works at Google.";
    let output = summarize_text(text);
    println!("{}", output);
}
