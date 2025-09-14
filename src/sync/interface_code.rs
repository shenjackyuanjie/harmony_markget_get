//! 用于全局共享并及时更新

use std::{
    sync::LazyLock,
    time::{Duration, Instant, UNIX_EPOCH},
};

use reqwest::Client;

pub static GLOBAL_CODE: LazyLock<CodeGenerater> = LazyLock::new(|| {
    let now = std::time::Instant::now();
    let client = reqwest::ClientBuilder::new().build().expect("faild to build client");
    CodeGenerater {
        last_update: now,
        token: None,
        update_interval: Duration::from_secs(60),
        client
    }
});

pub struct CodeGenerater {
    last_update: Instant,
    token: Option<String>,
    update_interval: Duration,
    client: Client
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
        const URL: &str = "https://web-drcn.hispace.dbankcloud.com/edge/webedge/getInterfaceCode";
        let unix_time: u64 = UNIX_EPOCH.elapsed().expect("wtf").as_millis() as u64;
        let response = self.client.post(URL)
            .header("Content-Type", "application/json")
            .header(
                "User-Agent",
                format!("get_huawei_market/{}", env!("CARGO_PKG_VERSION")),
            )
            .header("Interface-Code", format!("null_{unix_time}"))
            .send()
            .await.expect("faild to get token");
        if !response.status().is_success() {
            panic!("faild to get token");
        }
        let token = response.text().await.expect("faild to get token").trim_matches('\"').to_string();
        unsafe {
            // WARN: unsafe here
            let this = (self as *const Self as *mut Self).as_mut().unwrap();
            this.token = Some(token.clone());
            this.last_update = Instant::now();
        }
        token
    }
}
