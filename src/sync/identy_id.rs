//! 用于全局共享 identity id

use std::sync::{LazyLock, atomic::AtomicU32};

use colored::Colorize;

pub static GLOBAL_IDENTITY_ID: LazyLock<IdentityId> = LazyLock::new(|| {
    // 初始化全局 identity id
    IdentityId {
        id: uuid::Uuid::new_v4(),
        use_count: AtomicU32::new(0),
    }
});

pub struct IdentityId {
    id: uuid::Uuid,
    use_count: AtomicU32,
}

impl IdentityId {

    /// 获取形似 xxxxxxxxxxxxxxxx 的 identity id
    pub fn get_identity_id(&self) -> String {
        let use_count = self
            .use_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if use_count > 100 {
            // update
            let new_id = uuid::Uuid::new_v4();
            self.use_count
                .store(0, std::sync::atomic::Ordering::Relaxed);
            unsafe {
                let this = (self as *const Self as *mut Self).as_mut().unwrap();
                this.id = new_id;
            }
            println!("{} {}", "刷新 identity_id".on_blue(), format!("{:016x}", self.id).to_lowercase().replace("-", ""));
        }
        format!("{:016x}", self.id).to_lowercase().replace("-", "")
    }
}
