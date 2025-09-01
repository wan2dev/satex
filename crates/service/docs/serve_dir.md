# ServeDir

ServeDir 模块用于提供静态文件服务，支持目录的递归遍历和文件缓存。

## 配置

| 参数名                              | 默认值 | 描述                     |
|----------------------------------|-----|------------------------|
| path                             |     | 静态文件目录                 |
| buf_chunk_size                   |     | 缓冲区大小                  |
| append_index_html_on_directories |     | 是否在访问目录时自动添加index.html |

## 示例

- **完整配置模式**

```yaml
router:
  routes:
    - id: serve-dir-full
      matchers:
        - kind: ServeDir
          args:
            path: /srv/html
            buf_chunk_size: 65536
            append_index_html_on_directories: true
```

- **快捷配置**

```yaml
router:
  routes:
    - id: serve-dir-shortcut
      matchers:
        - ServeDir=/srv/html,65536,true
```