use tokenizers::Tokenizer;

pub fn tokenize(text: &str) -> Result<Vec<u32>, String> {
    let tokenizer = Tokenizer::from_pretrained("huggingface/llama3", None)
        .map_err(|e| format!("Tokenizer error: {:?}", e))?;

    let encoding = tokenizer.encode(text, true)
        .map_err(|e| format!("Encoding error: {:?}", e))?;

    Ok(encoding.get_ids().to_vec())
}

pub fn detokenize(tokens: &[u32]) -> Result<String, String> {
    let tokenizer = Tokenizer::from_pretrained("huggingface/llama3", None)
        .map_err(|e| format!("Tokenizer error: {:?}", e))?;

    let text = tokenizer.decode(tokens, true)
        .map_err(|e| format!("Decoding error: {:?}", e))?;