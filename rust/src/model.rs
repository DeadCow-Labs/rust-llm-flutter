use candle_core::{Device, DType, Result};
use candle_transformers::models::llama::{Llama, Config, Cache};
use candle_nn::VarBuilder;
use tokenizers::Tokenizer;
use std::path::Path;
use hf_hub::api::sync::ApiBuilder;
use serde_json::Value;

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

fn load_config_from_hub(model_id: &str) -> Result<Config> {
    let api = ApiBuilder::new()
        .with_progress(true)
        .build()
        .map_err(|e| candle_core::Error::Msg(format!("API error: {}", e)))?;
        
    let repo = api.model(model_id.to_string());
    let config_file = repo.get("config.json")
        .map_err(|e| candle_core::Error::Msg(format!("Download error: {}", e)))?;
        
    let config_str = std::fs::read_to_string(config_file)
        .map_err(|e| candle_core::Error::Msg(format!("Read error: {}", e)))?;
        
    let config_json: Value = serde_json::from_str(&config_str)
        .map_err(|e| candle_core::Error::Msg(format!("JSON parse error: {}", e)))?;
    
    // Optimize config for mobile inference
    let config = Config {
        hidden_size: config_json["hidden_size"]
            .as_i64()
            .ok_or_else(|| candle_core::Error::Msg("missing hidden_size".to_string()))? as usize,
        intermediate_size: config_json["intermediate_size"]
            .as_i64()
            .ok_or_else(|| candle_core::Error::Msg("missing intermediate_size".to_string()))? as usize,
        vocab_size: config_json["vocab_size"]
            .as_i64()
            .ok_or_else(|| candle_core::Error::Msg("missing vocab_size".to_string()))? as usize,
        num_hidden_layers: config_json["num_hidden_layers"]
            .as_i64()
            .ok_or_else(|| candle_core::Error::Msg("missing num_hidden_layers".to_string()))? as usize,
        num_attention_heads: config_json["num_attention_heads"]
            .as_i64()
            .ok_or_else(|| candle_core::Error::Msg("missing num_attention_heads".to_string()))? as usize,
        num_key_value_heads: config_json["num_key_value_heads"]
            .as_i64()
            .unwrap_or(config_json["num_attention_heads"].as_i64().unwrap_or(32)) as usize,
        rms_norm_eps: config_json["rms_norm_eps"]
            .as_f64()
            .unwrap_or(1e-5),
        rope_theta: config_json["rope_theta"]
            .as_f64()
            .unwrap_or(10000.0) as f32,
        use_flash_attn: false,  // Disabled flash attention since it's not compiled in
    };
    
    Ok(config)
}

pub fn load_model(model_name: &str) -> Result<Model> {
    let device = Device::Cpu;
    println!("Using device: {:?}", device);
    
    let model_dir = Path::new("models").join(model_name);
    std::fs::create_dir_all(&model_dir)?;
    
    let model_path = model_dir.join("model.safetensors");
    let tokenizer_path = model_dir.join("tokenizer.json");
    let config_path = model_dir.join("config.json");

    // Download files if they don't exist
    download_if_needed(model_name, "model.safetensors", &model_path)?;
    download_if_needed(model_name, "tokenizer.json", &tokenizer_path)?;
    download_if_needed(model_name, "config.json", &config_path)?;

    let tokenizer = Tokenizer::from_file(&tokenizer_path)
        .map_err(|e| candle_core::Error::Msg(format!("Failed to load tokenizer: {}", e)))?;

    let config = load_config_from_hub(model_name)?;
    
    // Use F16 with optimized cache
    let cache = Cache::new(true, DType::F16, &config, &device)?;
    
    // Load model with F16 precision
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[model_path], DType::F16, &device)?
    };
    
    let model = Llama::load(vb, &cache, &config)?;

    Ok(Model { model, tokenizer, cache, config })
}
