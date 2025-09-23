use tracing::{Level, event};

pub mod config;
pub mod datas;
pub mod db;
pub mod server;
pub mod sync;
pub mod utils;

fn main() -> anyhow::Result<()> {
    utils::init_log();
    // 加载配置
    let _ = config::Config::load()?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;
    event!(Level::INFO, "async rt built");
    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    // let (worker_send, worker_recv) = tokio::sync::oneshot::channel::<()>();

    // let worker = tokio::spawn(server::worker(worker_recv));

    // // 等待 ctrl + c
    // tokio::signal::ctrl_c().await?;

    // worker_send.send(()).unwrap();
    // worker.abort();
    // worker.await??;

    Ok(())
}
