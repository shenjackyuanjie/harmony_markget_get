//! 用于全局共享 identity id

use std::{
    sync::{LazyLock, atomic::AtomicBool},
    time::Instant,
};

use colored::Colorize;

use crate::sync::{TOKEN_UPDATE_INTERVAL, interface_code};

pub static GLOBAL_IDENTITY_ID: LazyLock<IdentityId> = LazyLock::new(|| {
    let now = std::time::Instant::now();
    // 初始化全局 identity id
    IdentityId {
        id: uuid::Uuid::new_v4(),
        last_update: now,
        updating: AtomicBool::new(false),
    }
});

pub struct IdentityId {
    id: uuid::Uuid,
    last_update: Instant,
    updating: AtomicBool,
}

impl IdentityId {
    /// 获取形似 xxxxxxxxxxxxxxxx 的 identity id
    pub fn get_identity_id(&self) -> String {
        // 优化：合并嵌套 if，减少 unsafe 使用，提升可读性
        if self.last_update.elapsed() > TOKEN_UPDATE_INTERVAL
            && !self.updating.load(std::sync::atomic::Ordering::Relaxed)
        {
            // 标记正在更新，防止并发问题
            self.updating.store(true, std::sync::atomic::Ordering::Relaxed);

            // 生成新的 id 和更新时间
            let new_id = uuid::Uuid::new_v4();
            let now = Instant::now();

            // 安全地更新 id 和 last_update
            let this = unsafe { (self as *const Self as *mut Self).as_mut().unwrap() };
            this.id = new_id;
            this.last_update = now;

            println!(
                "{} {}",
                "刷新 identity_id".on_blue(),
                format!("{:016x}", this.id).to_lowercase().replace("-", "")
            );
            println!(
                "{} 请稍候...",
                "正在更新 interface code".on_blue()
            );

            // 更新 token
            tokio::task::block_in_place(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    interface_code::GLOBAL_CODE.update_token().await;
                });
            });

            // 更新完成，重置 updating 标志
            this.updating.store(false, std::sync::atomic::Ordering::Relaxed);
        }
        format!("{:016x}", self.id).to_lowercase().replace("-", "")
    }
}
