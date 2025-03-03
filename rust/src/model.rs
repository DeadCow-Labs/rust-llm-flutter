use std::path::Path;
use candle_core::Device;
use candle_transformers::models::llama::Llama;

pub fn load_model(model_name: &str) -> Result<Llama, String> {
    let model_path = format!("./models/{}/model.bin", model_name);

    if !Path::new(&model_path).exists() {
        return Err("Model file not found!".to_string());
    }

    let device = Device::Cpu;
    match Llama::load_from_file(model_path, &device) {
        Ok(model) => Ok(model),
        Err(e) => Err(format!("Failed to load model: {:?}", e)),
    }
}
