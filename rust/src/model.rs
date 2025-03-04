use candle_core::{Device, DType, Result, Tensor};
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
    pub name: String,
}

impl Model {
    pub fn load_from_hub(model_name: &str) -> Result<Self> {
        let device = Device::Cpu;
        println!("Using device: {:?}", device);
        
        // Create base models directory first
        let models_dir = Path::new("models");
        if !models_dir.exists() {
            std::fs::create_dir_all(models_dir)
                .map_err(|e| candle_core::Error::Msg(format!("Failed to create models directory: {}", e)))?;
        }
        
        // Then create model-specific directory
        let model_dir = models_dir.join(model_name);
        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)
                .map_err(|e| candle_core::Error::Msg(format!("Failed to create model directory: {}", e)))?;
        }
        
        let model_path = model_dir.join("model.safetensors");
        let tokenizer_path = model_dir.join("tokenizer.json");
        let config_path = model_dir.join("config.json");

        // Download files if they don't exist
        Self::download_file(model_name, "model.safetensors", &model_path)?;
        Self::download_file(model_name, "tokenizer.json", &tokenizer_path)?;
        Self::download_file(model_name, "config.json", &config_path)?;

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to load tokenizer: {}", e)))?;

        let config = Self::load_config(&config_path)?;
        let cache = Cache::new(true, DType::F16, &config, &device)?;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[model_path], DType::F16, &device)?
        };
        
        let model = Llama::load(vb, &cache, &config)?;

        Ok(Model {
            model,
            tokenizer,
            cache,
            config,
            name: model_name.to_string(),
        })
    }

    pub fn load(path: &Path) -> Result<Self> {
        let device = Device::Cpu;
        println!("Using device: {:?}", device);
        
        let model_dir = path.parent().unwrap();
        let model_name = model_dir.file_name().unwrap().to_str().unwrap();
        
        let tokenizer_path = model_dir.join("tokenizer.json");
        let config_path = model_dir.join("config.json");

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to load tokenizer: {}", e)))?;

        let config = Self::load_config_from_file(&config_path)?;
        
        let cache = Cache::new(true, DType::F16, &config, &device)?;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[path.to_path_buf()], DType::F16, &device)?
        };
        
        let model = Llama::load(vb, &cache, &config)?;

        Ok(Model {
            model,
            tokenizer,
            cache,
            config,
            name: model_name.to_string(),
        })
    }

    pub fn download_if_needed(model_id: &str) -> Result<()> {
        // Create base models directory first
        let models_dir = Path::new("models");
        if !models_dir.exists() {
            std::fs::create_dir_all(models_dir)
                .map_err(|e| candle_core::Error::Msg(format!("Failed to create models directory: {}", e)))?;
        }
        
        // Then create model-specific directory
        let model_dir = models_dir.join(model_id);
        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)
                .map_err(|e| candle_core::Error::Msg(format!("Failed to create model directory: {}", e)))?;
        }
        
        let model_path = model_dir.join("model.safetensors");
        let tokenizer_path = model_dir.join("tokenizer.json");
        let config_path = model_dir.join("config.json");

        // Only download if files don't exist
        if !model_path.exists() {
            Self::download_file(model_id, "model.safetensors", &model_path)?;
        }
        if !tokenizer_path.exists() {
            Self::download_file(model_id, "tokenizer.json", &tokenizer_path)?;
        }
        if !config_path.exists() {
            Self::download_file(model_id, "config.json", &config_path)?;
        }

        Ok(())
    }

    fn download_file(model_id: &str, filename: &str, save_path: &Path) -> Result<()> {
        if !save_path.exists() {
            println!("Downloading {} for {}...", filename, model_id);
            
            // Ensure parent directory exists
            if let Some(parent) = save_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| candle_core::Error::Msg(format!("Failed to create directory: {}", e)))?;
                }
            }
            
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

    fn load_config(config_path: &Path) -> Result<Config> {
        let config_str = std::fs::read_to_string(config_path)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to read config: {}", e)))?;
            
        let config_json: Value = serde_json::from_str(&config_str)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to parse config: {}", e)))?;

        Ok(Config {
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
            use_flash_attn: false,
        })
    }

    fn load_config_from_file(path: &Path) -> Result<Config> {
        Self::parse_config_file(path)
    }

    fn parse_config_file(path: &Path) -> Result<Config> {
        let config_str = std::fs::read_to_string(path)
            .map_err(|e| candle_core::Error::Msg(format!("Read error: {}", e)))?;
            
        let config_json: Value = serde_json::from_str(&config_str)
            .map_err(|e| candle_core::Error::Msg(format!("JSON parse error: {}", e)))?;
        
        Ok(Config {
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
            use_flash_attn: false,
        })
    }

    pub fn run_inference(&self, input: &str) -> Result<String> {
        // Simpler chat format
        let formatted_input = format!("### Human: {}\n### Assistant:", input);
        let tokens = self.tokenizer.encode(formatted_input, true)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to tokenize: {}", e)))?;
        
        let input_ids = tokens.get_ids();
        println!("Input tokens: {:?}", input_ids);
        
        let input_tensor = Tensor::new(input_ids, &Device::Cpu)?
            .unsqueeze(0)?;
        println!("Input tensor shape: {:?}", input_tensor.shape());

        println!("Generating response...");
        let mut generated_tokens = Vec::new();
        let mut next_token = input_tensor;
        
        for i in 0..100 {
            let logits = self.model.forward(&next_token, i)?;
            let logits = logits.squeeze(0)?;
            let next_token_id = logits.argmax(0)?.to_vec0::<u32>()?;
            
            // Stop conditions
            if next_token_id == self.tokenizer.token_to_id("</s>").unwrap_or(0) 
               || next_token_id == self.tokenizer.token_to_id("### Human").unwrap_or(0)
               || next_token_id == self.tokenizer.token_to_id("###").unwrap_or(0) {
                break;
            }
            
            generated_tokens.push(next_token_id);
            next_token = Tensor::new(&[next_token_id], &Device::Cpu)?.unsqueeze(0)?;
        }

        // Decode and clean up the response
        let output = self.tokenizer.decode(&generated_tokens, true)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to decode: {}", e)))?;

        // Clean up the response more aggressively
        let cleaned = output
            .trim()
            .trim_start_matches("### Assistant:")
            .trim_start_matches("Assistant:")
            .trim_start_matches("The answer is:")
            .trim_start_matches("Answer:")
            .trim()
            .lines()
            .take_while(|line| !line.starts_with("###"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        Ok(cleaned)
    }
}
