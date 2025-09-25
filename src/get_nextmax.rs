use tracing::{Level, event};

pub mod config;
pub mod model;
pub mod db;
pub mod server;
pub mod sync;
pub mod utils;

fn main() -> anyhow::Result<()> {
    utils::init_log();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    let src_url = "https://nextmax.cn";

    let client = reqwest::ClientBuilder::new().build()?;
    event!(Level::INFO, "Starting...");
    let start_time = std::time::Instant::now();
    let response = client.get(format!("{src_url}/all_apps")).send().await?;
    let request_spent = start_time.elapsed();
    event!(Level::INFO, "HTTP请求已发送 - 耗时 {:.2?}", request_spent);
    let body = response.text().await?;
    let response_spent = start_time.elapsed();
    let body_spent = response_spent - request_spent;
    event!(
        Level::INFO,
        "HTTP响应已接收 - 总耗时 {:.2?}, 响应体接收耗时 {:.2?}",
        response_spent,
        body_spent
    );

    let app_card_match = r#"<div class="app-card" onclick="window.location.href='/app/"#;
    let app_lst = body
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| l.starts_with(app_card_match))
        .map(|l| {
            l.trim_start_matches(app_card_match)
                .trim_end_matches("\'\">")
                .parse::<u32>()
        });

    let app_lst = {
        let mut app_ids = Vec::new();
        for stuff in app_lst {
            match stuff {
                Ok(id) => app_ids.push(id),
                Err(e) => {
                    event!(Level::ERROR, "Failed to parse app ID: {}", e);
                    continue;
                }
            }
        }
        app_ids.sort_unstable();
        app_ids.dedup();
        app_ids
    };
    // 输出统计信息
    event!(Level::INFO, "处理完成 - 共计 {} 个APP", app_lst.len());
    // println!("{:?}", app_lst);
    // https://appgallery.huawei.com/app/detail?id=com.tencent.hm.qqmusic&amp;channelId=SHARE
    // com.tencent.hm.qqmusic
    let app_gallery_match = "https://appgallery.huawei.com/app/detail?id=";

    let mut apps = Vec::new();
    for app in app_lst {
        let request = client
            .get(format!("{}/app/{}", src_url, app))
            .send()
            .await?;
        let response = request.text().await?;
        if response.contains(app_gallery_match) {
            // 有信息
            let app_id = {
                let app_gallery_text = response
                    .split("\n")
                    .map(|l| l.trim())
                    .find(|l| l.contains(app_gallery_match))
                    .unwrap_or("");

                app_gallery_text
                    .replace(
                        r#"<a href="https://appgallery.huawei.com/app/detail?id="#,
                        "",
                    )
                    .replace(app_gallery_match, "")
                    .split("&amp;")
                    .next()
                    .unwrap_or("")
                    .to_string()
            };
            event!(Level::INFO, "页面 {} 找到了 appid {}", app, app_id);
            apps.push(app_id);
        } else {
            continue;
        }
    }

    // 写到json里
    let json = serde_json::to_string_pretty(&apps)?;
    std::fs::write("apps.json", json)?;

    Ok(())
}
