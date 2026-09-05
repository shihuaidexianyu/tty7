# Fork 主线与 Nightly

主仓库：[shihuaidexianyu/tty7](https://github.com/shihuaidexianyu/tty7)。
上游：[l0ng-ai/tty7](https://github.com/l0ng-ai/tty7)，保留来源、许可证和依赖的原有 pin。

- `main` 是本 fork 的日常开发、CI 和 Nightly 发布主线。
- 本地 `origin` 指向 fork；`upstream` 保留原仓库；默认推送到 `origin`。
- 原上游 Draft PR #775 保持独立，其分支不包含 fork 专属更新源修改。

## 更新与首次迁移

从本 fork 的 [Nightly Release](https://github.com/shihuaidexianyu/tty7/releases/tag/nightly)
手动安装一次。旧上游二进制中的更新地址不会因仓库变化而自行改变。

新配置默认选择 Nightly。已有配置的明确频道选择会保留；如果以前选择 Stable，安装后到
「设置 → 关于 → 更新频道」选择 Nightly。Stable 仍指向本 fork，但只有本 fork 发布稳定版后才有可用内容。
无需删除配置、工作区或会话数据。

客户端更新 API、发布页、远端 `tty7-server` 和 `checksums.txt` 下载地址，以及 Windows 安装器的
更新/支持链接，都从根 `Cargo.toml` 的 `workspace.package.repository` 获取仓库身份。
更新校验没有关闭，不会回退下载上游资产。

## 构建

在本 fork 的 Actions 中手动运行 Nightly，选择 `main`，或使用：

```sh
gh workflow run nightly.yml --repo shihuaidexianyu/tty7 --ref main
```

现有定时触发为每日北京时间 02:00；提交未变化且已发布时跳过。版本以 workspace 版本为下限，
存在标准稳定版 tag 时同时确保高于最后稳定版；没有稳定版 tag 的新 fork 也可以构建。
只有全部平台成功后才发布滚动 `nightly`，包含客户端包、四种远端服务器、`nightly.json` 与 SHA-256 清单。

Windows 安装器及 portable 包未签名，首次运行可能出现系统信任提示。未配置 Apple Developer ID
和公证 secrets 时，macOS 包按现有流程临时签名，不能宣称已公证或通过 Gatekeeper；不降低应用内签名校验。
这些打包信任条件与编译、资产完整性检查是不同的验收项。
