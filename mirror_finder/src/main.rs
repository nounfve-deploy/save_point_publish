use std::time::Duration;

use anyhow::anyhow;
use futures_util::StreamExt;
use tokio::fs;

#[tokio::main]
async fn main() {
    let winner = tokio::select! {
        res=fetch_sleep_forever_on_fail("")=>res,
        res=fetch_sleep_forever_on_fail("https://ghproxy.imciel.com/")=>res,
        res=fetch_sleep_forever_on_fail("https://cdn.akaere.online/")=>res,
        res=fetch_sleep_forever_on_fail("https://fastgit.cc/")=>res,
        _=tokio::time::sleep(Duration::from_secs(20))=>Err(anyhow!(TIMEOUT_TAG)),
    };
    let winner = match winner {
        Ok(val) => val,
        Err(err) => return eprintln!("{err}"),
    };
    println!("winner:{winner:?}");
    if winner.is_empty() {
        return;
    }
    let config = CONFIG_TEMPLATE.replace("{__winner__}", winner);
    fs::write("config.global.yaml", config).await.unwrap();
}

const CONFIG_TEMPLATE: &str = r#"url_rewrite:
  - org: https://github.com/
    new: {__winner__}https://github.com/
"#;

const TIMEOUT_TAG: &str = "!timeout!";
const TEST_LINK: &str = "https://github.com/microsoft/vscode/archive/refs/tags/1.138.0.zip";

async fn fetch_sleep_forever_on_fail(prefix: &str) -> anyhow::Result<&str> {
    let fetch = fetch_4mb_from(prefix).await;
    if fetch.is_err() {
        tokio::time::sleep(Duration::from_hours(2)).await;
    }
    fetch
}

async fn fetch_4mb_from(prefix: &str) -> anyhow::Result<&str> {
    let url = format!("{prefix}{TEST_LINK}");
    let resp = reqwest::get(url).await?;
    let mut resp = resp.bytes_stream();
    let mut size = 1024 * 1024 * 4;
    while let Some(Ok(chunk)) = resp.next().await {
        size -= chunk.len() as isize;
        if size < 0 {
            return Ok(prefix);
        }
    }
    Err(anyhow!("should never"))
}
