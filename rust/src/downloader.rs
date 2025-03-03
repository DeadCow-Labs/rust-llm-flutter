use std::fs::File;
use std::io::copy;
use std::path::Path;
use reqwest::blocking::get;

pub fn download_model(model_name: &str) -> Result<(), String> {
    let url = format!("https://huggingface.co/models/{}/model.bin", model_name);
    let save_path = format!("./models/{}/model.bin", model_name);

    if Path::new(&save_path).exists() {
        return Ok(()); // Skip if already downloaded
    }

    let mut response = get(&url).map_err(|e| format!("Request failed: {:?}", e))?;
    let mut file = File::create(&save_path).map_err(|e| format!("File error: {:?}", e))?;
    copy(&mut response, &mut file).map_err(|e| format!("Download error: {:?}", e))?;

    Ok(())
}
