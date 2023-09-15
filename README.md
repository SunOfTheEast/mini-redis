在 `volo-example` 目录下运行

示例：

- `cargo run --bin server 22222 -m 8080 9090 11111`
  - 运行主服务器（端口号 22222），其从节点端口号为：8080，9090，11111
- `cargo run --bin server 22222 -m`
  - 运行主服务器（端口号 22222），不带从节点
- `cargo run --bin server 8080 -s 22222`
  - 运行从服务器（端口号 8080），其主节点端口号为 22222
- `cargo run --bin server 7777 -p 8080 9090 11111`
  - 运行 Proxy（端口号 7777），负责的集群内有节点 8080，9090，11111
- `cargo run --bin client get zju 8080`
  - 运行客户端（端口号 8080），命令为 `get zju`
- `cargo run --bin client set zju 1897 8080`
  - 运行客户端（端口号 8080），命令为 `set zju 1897`
- 运行 `run.py` 会分析配置文件 `config.txt` 生成运行服务器的脚本 `run_servers.sh`