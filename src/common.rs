use clap::Parser;
use fred::{prelude::{RedisClient, ClientLike}, types::{RedisConfig, ServerConfig, ReconnectPolicy}};
use serde::{Serialize, Deserialize};
use serde_json::Value;

// 命令行参数封装
#[derive(Parser, Debug)]
#[command(
    author = "2857000511@qq.com",
    version = "0.1.0",
    about = "redis web ui manager...",
    name = "simple-redis-admin",
)]
pub struct Args {
    /// the service name for primary/main instances
    #[arg(short = 'n', long)]
    pub service_name: String,
    /// the redis's sentinel host, eg: 192.168.1.1:26379,192.168.1.2:26379,192.168.1.3:26379
    #[arg(short, long)]
    pub sentinel_host: String,
    /// the web ui service's port
    #[arg(short, long, default_value_t = 8080)]
    pub port: u32,
}

/// 初始化redis链接
pub async fn init_redis_client(service_name: &String, sentinel_host: &String) -> anyhow::Result<RedisClient> {
    // 解析sentinel host地址
    let hosts = sentinel_host.split(",").collect::<Vec<&str>>();
    let mut sentinel_host: Vec<fred::types::Server> = Vec::with_capacity(hosts.len());
    for host in hosts {
        let v = host.split(':').collect::<Vec<&str>>();
        let h = v[0];
        let port: u16 = v[1].parse::<u16>()?;
        sentinel_host.push(fred::types::Server::new(h, port));
    }
    let config = RedisConfig {
        server: ServerConfig::Sentinel {
            service_name: service_name.into(),
            hosts: sentinel_host.into(),
            #[cfg(feature = "sentinel-auth")]
            username: None,
            #[cfg(feature = "sentinel-auth")]
            password: None,
         },
         ..Default::default()
    };

    let policy = ReconnectPolicy::default();
    let client = RedisClient::new(config, None, Some(policy));
    client.connect();
    client.wait_for_connect().await?;
    tracing::info!("all right, redis client connected...");
    Ok(client)
}

// 返回的json
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl Default for Message {
    fn default() -> Self {
        Message::new()
    }
}

impl Message {
    pub fn new() -> Self {
        Message { code: 200, data: None, message: None }
    }
    pub fn set_code(&mut self, code: u16) -> &mut Self{
        self.code = code;
        self
    }
    pub fn set_data(&mut self, data: Value) -> &mut Self {
        self.data = Some(data);
        self
    }
    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

}