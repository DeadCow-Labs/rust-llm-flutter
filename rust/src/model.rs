use candle_core::{Device, DType, Result};
use candle_transformers::models::llama::{Llama, Config, Cache};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;
use std::path::Path;
use hf_hub::api::sync::ApiBuilder;

pub struct Model {
    pub model: Llama,
    pub tokenizer: Tokenizer,
    pub cache: Cache,
    pub config: Config,
}

pub fn download_if_needed(model_id: &str, filename: &str, save_path: &Path) -> Result<()> {
    if !save_path.exists() {
        println!("Downloading {} for {}...", filename, model_id);
        let api = ApiBuilder::new()
            .with_progress(true)
            .build()
            .map_err(|e| candle_core::Error::Msg(format!("API error: {}", e)))?;
            
        let repo = api.model(model_id.to_string());
        let file = repo.get(filename)
            .map_err(|e| candle_core::Error::Msg(format!("Download error: {}", e)))?;
            
        std::fs::copy(file, save_path)
            .map_err(|e| candle_core::Error::Msg(format!("Copy error: {}", e)))?;
    }
    Ok(())
}

pub fn load_model(model_name: &str) -> Result<Model> {
    let device = Device::Cpu;
    let model_dir = Path::new("models").join(model_name);
    std::fs::create_dir_all(&model_dir)?;
    
    let model_path = model_dir.join("model.safetensors");
    let tokenizer_path = model_dir.join("tokenizer.json");

    // Download files if they don't exist
    download_if_needed(
        "TinyLlama/TinyLlama-1.1B-Chat-v1.0",
        "model.safetensors",
        &model_path
    )?;
    download_if_needed(
        "TinyLlama/TinyLlama-1.1B-Chat-v1.0",
        "tokenizer.json",
        &tokenizer_path
    )?;

    // Load tokenizer
    let tokenizer = Tokenizer::from_file(&tokenizer_path)
        .map_err(|e| candle_core::Error::Msg(format!("Failed to load tokenizer: {}", e)))?;

    // Load model configuration
    let config = Config::config_7b_v2(false);
    
    // Create cache and load model
    let cache = Cache::new(true, DType::F32, &config, &device)?;
    
    // Fixed model loading with VarBuilder
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)?
    };
    let model = Llama::load(vb, &cache, &config)?;

    Ok(Model { model, tokenizer, cache, config })
}
