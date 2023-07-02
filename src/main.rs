use anyhow::Result;
use clap::Parser;
use std::fs::{create_dir, read_to_string, write};

mod args;
use args::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let contents = read_to_string(&args.file)?;
    let links: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let output_dir = args.output.clone();

    create_dir(&output_dir)?;

    // TODO: use JoinSet instead
    let mut tasks = Vec::new();

    for (index, url) in links.into_iter().enumerate() {
        let output_dir = output_dir.clone();
        let index = index + 1;

        let task = tokio::spawn(async move {
            download_image(output_dir, url, index).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}

async fn download_image(output: String, url: String, index: usize) {
    let response = reqwest::get(&url).await;
    match response {
        Ok(res) => {
            let image_bytes = res.bytes().await.unwrap();

            let file_name = format!("image_{}.jpg", index);
            let file_path = format!("./{}/", output) + &file_name;

            write(file_path, image_bytes).expect("Failed to save file in fs");

            print(
                format!("Downloaded image from: {}", url).as_str(),
                Args::parse().verbose,
            );
        }
        Err(err) => {
            eprintln!("Failed to download image from {}: {}", url, err);
        }
    }
}

fn print(s: &str, verbose: bool) {
    if verbose {
        println!("{}", s);
    }
}

