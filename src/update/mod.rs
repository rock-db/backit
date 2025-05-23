use serde::{Deserialize, Serialize};

use crate::utils::{download::download_simple_progress, error::ResultExt, paths};

#[derive(Deserialize, Serialize)]
pub struct Update {
    pub version: String,
    pub url: Vec<String>,
}

pub async fn update() {
    let update_dir = paths::update_config_dir();
    let update_file = update_dir.join("update_urls.json");

    if !update_file.exists() {
        download_simple_progress(
            //"https://backit.rock-db.jp/r/update/default_update_urls.json",
            "https://",
            &update_file,
        )
        .await
        .unwrap();
    }

    let updates: Update =
        serde_json::from_str(&std::fs::read_to_string(&update_file).unwrap_or_fail_with(
            "Failed to read update file",
            "Failed to read update_url file\nCaused by: ",
        ))
        .unwrap();

    for url in updates.url.iter() {
        let file_name = url.split('/').last().unwrap().to_string();
        let file_path = paths::updates_dir().join(&file_name);

        if !file_path.exists() {
            download_simple_progress(url, &file_path)
                .await
                .unwrap_or_fail("Failed to download update", "Failed to download update");
        }


    }
}
