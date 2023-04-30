// Crates
use rust_bert::bart::{
    BartConfigResources, BartMergesResources, BartModelResources, BartVocabResources,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use rust_bert::resources::RemoteResource;

// Loads the model
pub fn load_model(min_length: i64, max_length: i64) -> SummarizationModel {
    // Get all the necessary resources to create the model
    let model_type = ModelType::Bart;

    let config_resource = Box::new(RemoteResource::from_pretrained(
        BartConfigResources::DISTILBART_CNN_6_6,
    ));
    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        BartVocabResources::DISTILBART_CNN_6_6,
    ));
    let merges_resource = Box::new(RemoteResource::from_pretrained(
        BartMergesResources::DISTILBART_CNN_6_6,
    ));
    let model_resource = Box::new(RemoteResource::from_pretrained(
        BartModelResources::DISTILBART_CNN_6_6,
    ));

    // Create the configuration
    let summarization_config = SummarizationConfig {
        model_type,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource: Some(merges_resource),
        min_length,
        max_length: Some(max_length),
        ..Default::default()
    };

    // Return the model
    SummarizationModel::new(summarization_config).unwrap()
}

// Pipeline to summarize the text
pub fn summarize_text(text: &str) -> String {
    // Calculate the min and max length of the summary
    let min_length = 1;
    let max_length = text.len();

    // Check if the text is empty
    if text.is_empty() {
        return "No text was provided!".to_string();
    }

    let model = load_model(min_length, max_length as i64);
    let output = model.summarize(&[text]);
    output[0].to_string()
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_summary_output_length() {
        // Setup
        let text: &str =
            "Hello World! My name is John Doe and I live in the United States of America. I am a software engineer who works at Google.";
        let output = summarize_text(text);

        // Ensure the summary is the proper length
        let min_length = 1;
        let max_length = text.len();

        assert!(output.len() >= min_length && output.len() <= max_length);
    }

    #[test]
    fn test_summary_output_length_1_word() {
        // Setup
        let text: &str = "Hello";
        let output = summarize_text(text);

        // Ensure the summary is the proper length
        let min_length = 1;
        let max_length = text.len();

        assert!(output.len() >= min_length && output.len() <= max_length);
    }

    #[test]
    fn test_summary_output_length_0_words() {
        // Setup
        let text: &str = "";
        let output = summarize_text(text);

        assert_eq!(output, "No text was provided!");
    }

    #[test]
    fn test_summary_different_original() {
        // Setup
        let text: &str =
            "Hello World! My name is John Doe and I live in the United States of America. I am a software engineer who works at Google.";
        let output = summarize_text(text);

        // Ensure the summary is different from the original
        assert_ne!(output, text);
    }
}
