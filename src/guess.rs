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

    // C5765880207856366961
    // C5765880207852915863

    // let start = 

    Ok(())
}
