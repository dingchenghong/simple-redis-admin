simple-redis-admin

一个简单的带一个web页面的redis管理工具，项目用到的第三方库：salvo、tera

功能太简单，有需要的朋友，可自行改造


查看帮助
```
cargo run -- --help
Options:
  -n, --service-name <SERVICE_NAME>    the service name for primary/main instances
  -s, --sentinel-host <SENTINEL_HOST>  the redis's sentinel host, eg: 192.168.1.1:26379,192.168.1.2:26379,192.168.1.3:26379
  -p, --port <PORT>                    the web ui service's port [default: 8080]
  -h, --help                           Print help
  -V, --version                        Print version
```
运行起来，举例如下：
```
cargo run -- -n mymaster -s 192.168.8.1:26379,192.168.8.2:26379,192.168.8.3:26379
```
-n 表示主服务别名

-s 表示sentinel的主机
