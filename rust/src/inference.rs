use candle_core::{Device, Tensor, Result};
use crate::model::load_model;
use std::io::{self, Write};

pub fn run_inference(input: &str) -> Result<String> {
    // Use TinyLlama which is a small, mobile-friendly model
    let model = load_model("TinyLlama/TinyLlama-1.1B-Chat-v0.6")?;

    // Add a prompt template
    let prompt = format!("### Human: {}\n### Assistant:", input);
    
    let tokens = model.tokenizer.encode(prompt.as_str(), true)
        .map_err(|e| candle_core::Error::Msg(format!("Tokenizer error: {:?}", e)))?;
    
    println!("Input text: {}", prompt);
    println!("Input tokens: {:?}", tokens.get_ids());
    
    // Convert tokens to tensor and reshape
    let token_ids = tokens.get_ids();
    let input_tensor = Tensor::new(token_ids, &Device::Cpu)
        .map_err(|e| candle_core::Error::Msg(format!("Tensor error: {:?}", e)))?
        .reshape((1, token_ids.len()))?;
    
    println!("Input tensor shape: {:?}", input_tensor.shape());
    println!("Generating response...");

    // Generate multiple tokens
    let mut generated_text = String::new();
    let mut current_input = input_tensor;
    let mut position = 0;
    let mut all_tokens = Vec::new();

    for i in 0..50 {  // Generate up to 50 tokens
        // Print progress
        print!("\rToken {}/50", i + 1);
        io::stdout().flush().unwrap();

        let output = model.model.forward(&current_input, position)
            .map_err(|e| candle_core::Error::Msg(format!("Inference error: {:?}", e)))?;
        
        let logits = output.squeeze(0)?;
        
        // Get the token with highest probability
        let next_token_id = logits.argmax(0)?
            .to_scalar::<u32>()
            .map_err(|e| candle_core::Error::Msg(format!("Scalar conversion error: {:?}", e)))?;
        
        // Stop if we hit the end token
        if next_token_id == 2 {  // EOS token
            break;
        }
        
        // Add token to our collection
        all_tokens.push(next_token_id);
        
        // Convert all tokens to text to maintain spacing
        let partial_text = model.tokenizer.decode(&all_tokens, true)
            .map_err(|e| candle_core::Error::Msg(format!("Decoding error: {:?}", e)))?;
        
        print!("\rPartial response: {}", partial_text);
        io::stdout().flush().unwrap();
        
        // Prepare next input: only use the last token
        current_input = Tensor::new(&[next_token_id], &Device::Cpu)?
            .reshape((1, 1))?;
        position += 1;
        
        // Break if we generate a newline after some content
        if partial_text.len() > 10 && next_token_id == 13 {
            break;
        }
    }

    // Get final text from all tokens
    let final_text = model.tokenizer.decode(&all_tokens, true)
        .map_err(|e| candle_core::Error::Msg(format!("Decoding error: {:?}", e)))?;

    println!("\nFinal response: {}", final_text);
    Ok(final_text)
}
