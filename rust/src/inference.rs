use candle_core::{Device, Tensor};
use crate::model::load_model;

pub fn run_inference(input: &str) -> Result<String, String> {
    let model = load_model("llama3").map_err(|e| e.to_string())?;

    let tokens = model.tokenizer.encode(input, true)
        .map_err(|e| format!("Tokenizer error: {:?}", e))?;
    
    // Convert tokens to tensor
    let token_ids = tokens.get_ids();
    let input_tensor = Tensor::new(token_ids, &Device::Cpu)
        .map_err(|e| format!("Tensor error: {:?}", e))?;

    // Run model inference with position index 0
    let output = model.model.forward(&input_tensor, 0)
        .map_err(|e| format!("Inference error: {:?}", e))?;
    
    // Convert output back to text
    let output_ids = output.to_vec1::<u32>()
        .map_err(|e| format!("Output conversion error: {:?}", e))?;
    
    model.tokenizer.decode(&output_ids, true)
        .map_err(|e| format!("Decoding error: {:?}", e))
}
