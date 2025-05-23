use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::{io::Write, path::Path};

pub async fn download_simple_progress<P: AsRef<Path>>(
    url: &str,
    file_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let file_path = file_path.as_ref();

    // Content-Lengthヘッダーからファイルサイズを取得
    let total_size = response.content_length().unwrap_or(0);

    // プログレスバーを設定
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes:>7}/{total_bytes:7} {bytes_per_sec} {msg}")
        .unwrap()
        .progress_chars("##-"));
    pb.set_message(format!("Downloading {}", file_path.display()));

    // 全データを一度に取得
    let content = response.bytes().await?;

    // プログレスバーを更新
    pb.set_position(content.len() as u64);

    // ファイルに書き込み
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(&content)?;

    pb.finish_with_message(format!("Downloaded {} successfully!", file_path.display()));
    Ok(())
}

pub async fn download_chunked_progress<P: AsRef<Path>>(
    url: &str,
    file_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let file_path = file_path.as_ref();

    // HEADリクエストでファイルサイズを事前取得
    let head_response = client.head(url).send().await?;
    let total_size = head_response.content_length().unwrap_or(0);

    // プログレスバーを設定
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes:>7}/{total_bytes:7} {bytes_per_sec} {msg}")
        .unwrap()
        .progress_chars("##-"));
    pb.set_message(format!("Downloading {}", file_path.display()));

    let mut file = std::fs::File::create(file_path)?;
    let mut downloaded = 0u64;
    let chunk_size = 8192; // 8KB chunks

    // Range requestsを使用してチャンクごとにダウンロード
    while downloaded < total_size {
        let end = std::cmp::min(downloaded + chunk_size - 1, total_size - 1);
        let range = format!("bytes={}-{}", downloaded, end);

        let chunk_response = client.get(url).header("Range", range).send().await?;

        let chunk = chunk_response.bytes().await?;
        file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);

        // 少し待機（オプション）
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    pb.finish_with_message(format!("Downloaded {} successfully!", file_path.display()));
    Ok(())
}
