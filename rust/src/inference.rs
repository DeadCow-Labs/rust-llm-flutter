use candle_core::Tensor;
use candle_transformers::models::llama::Llama;

pub fn run_inference(input: &str) -> Result<String, String> {
    let model = load_model("llama3").map_err(|e| e.to_string())?;

    let tokens = model.tokenizer.encode(input, true)
        .map_err(|e| format!("Tokenizer error: {:?}", e))?;

    let output = model.generate_text(&tokens, 50) // Limit output tokens
        .map_err(|e| format!("Inference error: {:?}", e))?;

    Ok(output)
}
