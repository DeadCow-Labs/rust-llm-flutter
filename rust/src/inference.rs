use candle_core::{Device, Tensor, Result};
use std::io::{self, Write};
use std::time::Instant;
use crate::MODEL;  // Import the global MODEL from lib.rs

pub fn run_inference(input: &str) -> Result<String> {
    let start_time = Instant::now();
    
    // Get the model from global state
    let model_lock = MODEL.lock().unwrap();
    let model = model_lock.as_ref()
        .ok_or_else(|| candle_core::Error::Msg("Model not loaded".to_string()))?;

    println!("Got model from global state, running inference...");
    
    let prompt = format!("### Human: {}\n### Assistant:", input);
    
    let tokens = model.tokenizer.encode(prompt.as_str(), true)
        .map_err(|e| candle_core::Error::Msg(format!("Tokenizer error: {:?}", e)))?;
    
    println!("Input text: {}", prompt);
    println!("Input tokens: {:?}", tokens.get_ids());
    
    let token_ids = tokens.get_ids();
    let input_tensor = Tensor::new(token_ids, &Device::Cpu)?
        .reshape((1, token_ids.len()))?;
    
    println!("Input tensor shape: {:?}", input_tensor.shape());
    println!("Generating response...");

    let mut all_tokens = Vec::with_capacity(token_ids.len() + 50);
    all_tokens.extend_from_slice(token_ids);
    
    let mut current_input = input_tensor;
    let mut position = 0;
    let mut total_tokens = 0;
    let mut token_times = Vec::new();

    for i in 0..50 {
        let token_start = Instant::now();
        
        let output = model.model.forward(&current_input, position)?;
        let logits = output.squeeze(0)?;
        
        let next_token_id = logits.argmax(0)?
            .to_scalar::<u32>()?;
        
        if next_token_id == 2 {
            break;
        }
        
        all_tokens.push(next_token_id);
        total_tokens += 1;
        
        let token_time = token_start.elapsed().as_millis();
        token_times.push(token_time);
        
        let avg_time = token_times.iter().sum::<u128>() / token_times.len() as u128;
        print!("\rToken {}/50 ({}ms/token avg)", i + 1, avg_time);
        io::stdout().flush().unwrap();
        
        current_input = Tensor::new(&[next_token_id], &Device::Cpu)?
            .reshape((1, 1))?;
        position += 1;
        
        if all_tokens.len() > token_ids.len() + 10 && next_token_id == 13 {
            break;
        }
    }

    let final_text = model.tokenizer.decode(&all_tokens[token_ids.len()..], true)
        .map_err(|e| candle_core::Error::Msg(format!("Decoding error: {:?}", e)))?;

    let total_time = start_time.elapsed().as_millis();
    println!("\n\nStats:");
    println!("Total time: {}ms", total_time);
    println!("Total tokens generated: {}", total_tokens);
    println!("Average time per token: {}ms", total_time / total_tokens as u128);
    println!("Tokens per second: {:.2}", (total_tokens as f64 * 1000.0) / total_time as f64);

    Ok(final_text)
}
