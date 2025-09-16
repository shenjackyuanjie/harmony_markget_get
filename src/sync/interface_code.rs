//! 用于全局共享并及时更新

use std::{
    sync::LazyLock,
    time::{Duration, Instant, UNIX_EPOCH},
};

use colored::Colorize;
use reqwest::Client;

pub static GLOBAL_CODE: LazyLock<CodeGenerater> = LazyLock::new(|| {
    let now = std::time::Instant::now();
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("faild to build client");
    CodeGenerater {
        last_update: now,
        token: None,
        update_interval: Duration::from_secs(60),
        client,
    }
});

pub struct CodeGenerater {
    last_update: Instant,
    token: Option<String>,
    update_interval: Duration,
    client: Client,
}

impl CodeGenerater {
    pub async fn get_token(&self) -> String {
        // 检查更新时间
        match self.token.as_ref() {
            Some(token) => {
                if self.last_update.elapsed() > self.update_interval {
                    return self.update_token().await;
                }
                token.clone()
            }
            None => {
                return self.update_token().await;
            }
        }
    }

    /// 跟 get_token 的区别就是在token后面拼了一个 _unix time
    pub async fn get_full_token(&self) -> String {
        let unix_time: u64 = UNIX_EPOCH.elapsed().expect("wtf").as_millis() as u64;
        let token = self.get_token().await;
        format!("{}_{unix_time}", token)
    }

    // fn clone_token(&self) -> String {
    //     self.token.clone().expect("wtf, no token")
    // }

    pub async fn update_token(&self) -> String {
        println!("{}", "正在刷新 interface code".blue());
        const URL: &str = "https://web-drcn.hispace.dbankcloud.com/edge/webedge/getInterfaceCode";
        const MAX_RETRIES: usize = 3;

        let token = {
            let mut retry_count = 0;
            loop {
                if retry_count >= MAX_RETRIES {
                    panic!("达到最大重试次数，无法获取token");
                }

                let unix_time: u64 = UNIX_EPOCH.elapsed().expect("wtf").as_millis() as u64;
                let response_result = self
                    .client
                    .post(URL)
                    .header("Content-Type", "application/json")
                    .header(
                        "User-Agent",
                        format!("get_huawei_market/{}", env!("CARGO_PKG_VERSION")),
                    )
                    .header("Interface-Code", format!("null_{unix_time}"))
                    .send()
                    .await;

                match response_result {
                    Ok(response) => {
                        if !response.status().is_success() {
                            println!("{}", format!("请求失败，状态码: {}，正在重试 ({}/{})",
                                response.status(), retry_count + 1, MAX_RETRIES).yellow());
                            retry_count += 1;
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }

                        match response.text().await {
                            Ok(text) => {
                                let token = text.trim_matches('\"').to_string();
                                break token;
                            }
                            Err(e) => {
                                println!("{}", format!("解析响应失败: {}，正在重试 ({}/{})",
                                    e, retry_count + 1, MAX_RETRIES).yellow());
                                retry_count += 1;
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", format!("发送请求失败: {}，正在重试 ({}/{})",
                            e, retry_count + 1, MAX_RETRIES).yellow());
                        retry_count += 1;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                }
            }
        };

        unsafe {
            // WARN: unsafe here
            let this = (self as *const Self as *mut Self).as_mut().unwrap();
            this.token = Some(token.clone());
            this.last_update = Instant::now();
        }
        token
    }
}
