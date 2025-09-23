//! 用于全局共享 identity id

use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use colored::Colorize;

use crate::sync::{TOKEN_UPDATE_INTERVAL, interface_code};

pub static GLOBAL_IDENTITY_ID: LazyLock<IdentityId> = LazyLock::new(|| {
    let now = std::time::Instant::now();
    // 初始化全局 identity id
    IdentityId {
        id: uuid::Uuid::new_v4(),
        last_update: now,
    }
});

pub struct IdentityId {
    id: uuid::Uuid,
    last_update: Instant,
}

impl IdentityId {
    /// 获取形似 xxxxxxxxxxxxxxxx 的 identity id
    pub fn get_identity_id(&self) -> String {
        if self.last_update.elapsed() > TOKEN_UPDATE_INTERVAL {
            // update token
            let new_id = uuid::Uuid::new_v4();
            let now = Instant::now();
            unsafe {
                let this = (self as *const Self as *mut Self).as_mut().unwrap();
                this.id = new_id;
                this.last_update = now;
            }
            println!(
                "{} {}",
                "刷新 identity_id".on_blue(),
                format!("{:016x}", self.id).to_lowercase().replace("-", "")
            );
            tokio::task::block_in_place(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    interface_code::GLOBAL_CODE.update_token().await;
                });
            });
        }
        format!("{:016x}", self.id).to_lowercase().replace("-", "")
    }
}
