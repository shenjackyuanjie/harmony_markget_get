pub mod config;
pub mod datas;
pub mod db;
pub mod server;
pub mod sync;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    let (worker_send, worker_recv) = tokio::sync::oneshot::channel::<()>();

    let worker = tokio::spawn(server::worker(config.clone(), worker_recv));

    // 等待 ctrl + c
    tokio::signal::ctrl_c().await?;

    worker_send.send(()).unwrap();
    worker.await??;

    Ok(())
}
