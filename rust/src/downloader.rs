use std::path::Path;
use hf_hub::api::sync::ApiBuilder;
use std::fs;

pub fn download_if_needed(repo_id: &str, filename: &str, save_path: &Path) -> candle_core::Result<()> {
    // Skip if file already exists
    if save_path.exists() {
        return Ok(());
    }

    // Create parent directories if they don't exist
    if let Some(parent) = save_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| candle_core::Error::Msg(format!("Failed to create dirs: {}", e)))?;
    }

    // Download from Hugging Face
    let api = ApiBuilder::new()
        .with_progress(true)
        .build()
        .map_err(|e| candle_core::Error::Msg(format!("API error: {}", e)))?;

    let repo = api.model(repo_id.to_string());
    let file = repo.get(filename)
        .map_err(|e| candle_core::Error::Msg(format!("Download error: {}", e)))?;

    // Copy downloaded file to target location
    fs::copy(&file, save_path)
        .map_err(|e| candle_core::Error::Msg(format!("Copy error: {}", e)))?;

    Ok(())
}
