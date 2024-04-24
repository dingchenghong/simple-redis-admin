use salvo::{Router, serve_static::StaticDir, affix};
use tera::Tera;
use anyhow::Result;

use crate::{controller::*, Args, common::init_redis_client};

// 初始化路由
pub async fn init(args: Args) -> Result<Router> {
    tracing::info!("redis-admin init, port: {}", args.port);
    tracing::info!("redis-admin init, service's name: {}", args.service_name);
    tracing::info!("redis-admin init, sentinel host: {}", args.sentinel_host);
    let redis_client = init_redis_client(&args.service_name, &args.sentinel_host).await?;
    let tera = Tera::new("templates/**/*")?;
    let router = Router::new()
        .push(Router::with_path("static/<**path>").get(StaticDir::new(["static/"])))
        .hoop(affix::inject(tera))
        .hoop(affix::inject(redis_client))
        .get(index)
        .push(Router::with_path("get").post(redis_get))
        .push(Router::with_path("scan").post(scan))
        .push(Router::with_path("set").post(set))
        .push(Router::with_path("delete").post(delete))
        ;
    Ok(router)
}