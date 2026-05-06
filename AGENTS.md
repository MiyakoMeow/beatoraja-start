# 此仓库之 AGENTS 准则

Rust 项目（Cargo edition 2024）。beatoraja BMS 启动器——寻 JRE 之 `java`/`javaw`，以 `-Xms1g -Xmx8g -XX:+UseZGC` 起 `beatoraja.jar`。

## 1. 零外部依赖

`[dependencies]` 留空。加 crate 前，必先证其功能确需引入。路径查找与 Command 构建，当亲手为之，勿假外物。

## 2. 提交前 CI 必过

- `cargo build --all-features`（编译）
- `cargo clippy --all-features -- -D warnings`
- `cargo fmt --check`
- `cargo deny check`

三平台（ubuntu / windows / macOS）皆须通过。

## 3. Conventional Commits

类型：`feat` `fix` `refactor` `chore` `docs` `perf` `style` `test`。可选 scope：`(ci)` `(deps)` `(release)`。release-plz 据此自动生成 CHANGELOG。

## 4. 跨平台用 `#[cfg]`

平台差异以成对 `#[cfg]` 处理：

```rust
#[cfg(target_os = "windows")]
fn find_java() -> Option<PathBuf> { ... }

#[cfg(not(target_os = "windows"))]
fn find_java() -> Option<PathBuf> { ... }
```

## 5. 二进制文件二

- `src/bin/spawn.rs` → `beatoraja-start`（spawn，不待）
- `src/bin/non-spawn.rs` → `beatoraja-start-with-log`（wait，暂停以阅日志）

## 6. 错误处理

`.expect("message")`——直 panic。启动器无恢复路径。零依赖优先于优雅降级。

## 7. 附注

- 内置 JRE 为 21.0.1+12-LTS（OpenJDK）。
- 发布自动化：release-plz 建版本 PR → GitHub Release → 上传 binary。
