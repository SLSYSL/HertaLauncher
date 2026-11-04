# 贡献指南

## 环境要求

- Rust 1.92+
- Git

## 常用命令

- `cargo dev`: 带 `dev-tools` 运行
- `cargo run`: 运行程序
- `cargo check`: 快速检查（`.slint` 一并编译）
- `cargo build --release`: 发布构建

## dev-tools

`dev-tools` 启用后可使用 Slint 的 MCP 和实时预览调试能力

- **编译期**（`build.rs`）：`SLINT_LIVE_PREVIEW=1`、`SLINT_EMIT_DEBUG_INFO=1`
- **运行期**（`main.rs`）：设置 `SLINT_MCP_PORT`，未设置时由本项目回退到 `50302`

| 能力 | Cargo feature | 环境变量 | 作用 |
| --- | --- | --- | --- |
| MCP 服务器 | `slint/mcp` | `SLINT_MCP_PORT`（可选，默认 `50302`） | 让 MCP 客户端通过 HTTP 检查并操作运行中的 UI |
| 实时预览 | `slint/live-preview` | `SLINT_LIVE_PREVIEW=1`（自动注入） | `.slint` 文件保存后 UI 秒级热重载 |

### MCP 服务器

默认监听 `http://localhost:50302/mcp`，需要换端口时：

```sh
SLINT_MCP_PORT=8080 cargo dev
```

## 编辑器配置

`.vscode/settings.json`、`.zed/settings.json` 已把 `@kit/*` 映射到 `target/vendor/slint-kit-slint/ui`

该目录不入库，首次 clone 或 `cargo clean` 后按需重建：

```bash
cargo vendor --locked target/vendor
```

> [!WARNING]
> 请勿采纳 `replace-with = "vendored-sources"`，本仓库不用 vendored 源参与构建，仅用于编辑器解析 `.slint` 导入路径

## UI 约定

- 优先使用 [Slint Kit 组件](https://slint-kit.dev/components)，确认无法满足需求再自定义
- 自定义组件使用 Slint Kit 的 `KitPressable` 等基元为底座，继承其 a11y 处理逻辑
- 颜色、间距、圆角等语义从 `Store.theme` Token 组读取，不要硬编码字面量像素与十六进制色值
- 可变列表、选中项、页面路由等状态放 `ui/store.slint` 的 `Store`

## 依赖与许可

新增或升级依赖后跑一次许可门禁（白名单在 `deny.toml`，白名单外一律拒绝）：

```sh
cargo licenses # = cargo deny check licenses，离线可用
```

需要一次性装工具：`cargo install cargo-deny --locked`（独立二进制，不进 `Cargo.toml`）。
被拦下时：确认该许可与 `GPL-3.0-or-later` 兼容 → 加进 `deny.toml` 的 `allow`；不兼容 → 换依赖。

`cargo deny check` 会额外查安全公告（需能访问 GitHub 拉 RustSec 库，拉不到会降级为警告）；
传递依赖的「无人维护」提示已按 `deny.toml` 忽略，漏洞与 yank 不忽略。

## 贡献许可

你的贡献以 **GPL-3.0-or-later** 授权（inbound = outbound），无需签署 CLA。

- 请勿引入与本项目许可不兼容的第三方代码或素材；新增图标、字体、游戏素材前先确认授权
- 本项目源码**不加逐文件许可头**，许可与版权声明以根目录 `LICENSE` 和 `README.md` 为准；
  文件被单独复制时仍受 `GPL-3.0-or-later` 约束.
- 若修改了第三方来源的文件（vendored 或 fork 的代码），在文件头注明修改内容与日期
  （GPL-3.0 §5(a) 要求）：

  ```rust
  // Modifications:
  // 2026-04-12 Your Name <you@example.com> <一句话说明改动内容>
  ```

- 建议提交时加 DCO 签名：`git commit -s`，会在 commit message 末尾自动追加
  `Signed-off-by: Your Name <you@example.com>`

## 分发

对外分发（包括只分发编译好的二进制）时：

- 随附 `LICENSE`，并提供对应源码的获取途径（tag / 归档链接）
- 不得附加与 GPL 冲突的额外限制
- 内嵌素材的许可原文（字体、图标）随上游 `slint-kit` 包分发；要一并随包附上就 `cp` 一份：

  ```sh
  cp target/vendor/slint-kit-slint/THIRD-PARTY-NOTICES.md .
  ```
