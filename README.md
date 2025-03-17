# {{project-name}}

{{project_description}}

## 运行

```bash
cargo run
```

## 配置

配置文件位于`config.toml`，可设置以下参数：

- 服务器主机：`{{default_host}}`
- 服务器端口：`{{default_port}}`

## Docker

构建并推送Docker镜像：

```bash
make all
```

## API

- `GET /`: 返回欢迎信息
- `POST /upload`: 上传文件接口