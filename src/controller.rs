use anyhow::Result;
use chrono::Local;
use fred::prelude::*;
use fred::types::Scanner;
use futures::stream::StreamExt;
use salvo::Depot;
use salvo::prelude::*;
use tera::Context;
use tera::Tera;
use crate::common::Message;
use crate::error::Error;


#[handler]
pub async fn index(depot: &mut Depot) -> Result<Text<String>, Error> {
    let tera = depot.obtain::<Tera>().unwrap().to_owned();
    let mut context = Context::new();
    let year = Local::now().format("%Y").to_string();
    context.insert("year", &year);
    let response = tera.render("index.html", &context)?;
    Ok(Text::Html(response))
}

// 根据key关键字,搜索所有相关的key
#[handler]
pub async fn scan(depot: &mut Depot, req: &mut Request) -> Result<Json<Message>, Error> {
    // let key = req.query::<String>("key").unwrap_or_default();
    // let key = req.params().get("key").cloned().unwrap_or_default();
    let mut key = String::new();
    if let Some(k) = req.form::<String>("key").await {
        key = k;
    }
    tracing::info!("in scan key: {}", key);
    let client = depot.obtain::<RedisClient>().unwrap().to_owned();

    // 如果key中带有*符号，则进行scan操作,否则即获取key的value
    if key.contains("*") {
        tracing::info!("in scan, it is search opt...");
        let mut scan_stream = client.scan(key, None, None);
        let mut keys_vec: Vec<String> = Vec::new();
        while let Some(result) = scan_stream.next().await {
            let mut page = result.expect("SCAN failed with error");
            if let Some(keys) = page.take_results() {
                // create a client from the scan result, reusing the existing connection(s)
                // let client = page.create_client();
          
                for key in keys.into_iter() {
                  // let value: RedisValue = client.get(&key).await?;
                  // println!("Scanned {} -> {:?}", key.as_str_lossy(), value);
                //   println!("Scanned: {}", key.as_str_lossy());
                  // buffer.push((key, value));
                  keys_vec.push(key.as_str_lossy().to_string());
                }
              }
          
              // move on to the next page now that we're done reading the values. or move this before we call `get` on each key
              // to scan results in the background as quickly as possible.
              let _ = page.next();
        }
        let mut message = Message::new();
        message.set_data(keys_vec.into_iter().collect());
        Ok(Json(message))
    } else {
        tracing::info!("in scan, it is get opt...");
        let val: Option<String> = client.get(&key).await?;
        let mut data = String::new();
        if let Some(val) = val {
            data = format!("{}", val);
        } else {
            data = "没有找到相应的值,可能是该key不存在".into();
        }
        let mut message = Message::new();
        message.set_code(201);
        message.set_data(serde_json::Value::String(data));
        Ok(Json(message))
    }


}

#[handler]
pub async fn redis_get(depot: &mut Depot, req: &mut Request) -> Result<String, Error> {
    let key = req.query::<String>("key").unwrap_or_default();
    println!("key is: {}", key);
    let client = depot.obtain::<RedisClient>().unwrap().to_owned();
    // get the key
    let val: Option<String> = client.get(&key).await?;
    println!("key: {}, val: {:?}", key, val);

    // let _ = client.quit().await?;
    Ok("ok".to_string())
}

// 设置值
#[handler]
pub async fn set(depot: &mut Depot, req: &mut Request, resp: &mut Response) {
    let mut key = String::new();
    let mut val = String::new();
    if let Some(k) = req.form::<String>("key").await {
        key = k;
    }
    if let Some(v) = req.form::<String>("val").await {
        val = v;
    }
    tracing::info!("key: {}, val: {}", key, val);
    let client = depot.obtain::<RedisClient>().unwrap().to_owned();
    // let _: () = client.set("foo", "bar", None, None, false).await;
    match client.set::<String, _, _>(key, val, None, None, false).await {
        Ok(_) => {
            resp.render(Json(Message::new()));
        },
        Err(e) => {
            tracing::error!("in set, error happen, error: {}", e);
            let mut message = Message::new();
            message.set_code(400);
            message.set_message("failed...".into());
            resp.render(Json(message));
        }
    }
}

#[handler]
pub async fn delete(depot: &mut Depot, req: &mut Request, resp: &mut Response) {
    let mut key = String::new();
    if let Some(k) = req.form::<String>("key").await {
        key = k;
    }
    tracing::info!("in delete, key: {}", key);
    let client = depot.obtain::<RedisClient>().unwrap().to_owned();
    match client.del::<String, String>(key).await {
        Ok(_) => {
            resp.render(Json(Message::new()));
        },
        Err(e) => {
            tracing::error!("in delete, error happen, error: {}", e);
            let mut message = Message::new();
            message.set_code(400);
            message.set_message("failed...".into());
            resp.render(Json(message));
        }
    }
}
