mod common;
mod error;
mod controller;
mod router;
use clap::{Parser};
use salvo::{prelude::TcpListener, prelude::Server};
use crate::{router::init, common::Args};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    tracing_subscriber::fmt().init();
    let args = Args::parse();
    let port = args.port;
    let router = init(args).await?;
    let bind_str = format!("0.0.0.0:{}", port);
    tracing::info!("Listening on http://{}", bind_str);
    Server::new(TcpListener::bind(&bind_str))
        .serve(router)
        .await;
    Ok(())
}
