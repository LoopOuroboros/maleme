# SOP: 发布 maleme 到 npmjs

日期：2026-04-14

这份 SOP 约束的是当前仓库已经存在的发布链路：

- Rust 主版本来自 `Cargo.toml`
- npm 包版本通过 `node scripts/npm/sync-packages.mjs` 从 `Cargo.toml` 同步
- GitHub Actions 的 `publish-npm` workflow 只会在 `v*` tag push 时触发

## 结论先说

要发 npm，不是只推 `main`。

真正会触发发布的是：

1. 把版本号改对
2. 提交到 `main`
3. 打一个与 `Cargo.toml` 完全一致的 tag，例如 `v0.1.5`
4. 推送这个 tag

如果只 push `main`，不会发包。

## 发布前检查

每次发包前先确认：

1. 当前在最新 `main`
2. 工作区干净
3. `Cargo.toml` 里的版本号是你这次要发的新版本
4. 这个版本还没有被 npmjs 发布过
5. 这个 tag 还不存在于远端

建议命令：

```bash
git fetch origin
git checkout main
git pull --ff-only origin main
git status -sb
git tag --list 'v*' --sort=version:refname
```

## 正确发布流程

### 1. 更新版本号

直接修改 `Cargo.toml`：

```toml
version = "0.1.5"
```

### 2. 同步 npm 包版本

```bash
node scripts/npm/sync-packages.mjs
```

这一步会同步：

- `npm/main/package.json`
- `npm/platforms/*/package.json`

### 3. 运行校验

至少跑一遍：

```bash
cargo test
```

需要的话再补本地打包校验：

```bash
cargo build --release
```

### 4. 提交版本升级

```bash
git add Cargo.toml Cargo.lock npm/main/package.json npm/platforms/*/package.json
git commit -m "bump version 0.1.5"
```

### 5. 推送 main

```bash
git push origin main
```

### 6. 打 tag

注意：tag 必须和 `Cargo.toml` 版本完全一致。

如果版本是 `0.1.5`，tag 必须是：

```bash
git tag v0.1.5
git push origin refs/tags/v0.1.5
```

## 发布后检查

### 查看 workflow 状态

```bash
gh run list --workflow publish-npm --limit 5
gh run view <run-id>
```

### 预期结果

你应该看到：

- `publish-npm`
- ref 为 `v0.1.5`
- 5 个平台包 job
- 最后 `publish-main` 成功

## 常见错误

### 错误 1：只 push 了 main，没有 push tag

表现：

- 代码上了 GitHub
- npmjs 没有新版本
- `publish-npm` 没有新 run

原因：

- `.github/workflows/publish-npm.yml` 只监听 tag push

### 错误 2：tag 和 Cargo 版本不一致

表现：

- workflow 在 `Verify release tag` 直接失败

典型报错：

```text
Tag v0.1.4 does not match Cargo.toml version 0.1.2
```

处理方式：

- 不要硬修旧 tag
- 直接把版本 bump 到一个新的未发布版本
- 例如从 `0.1.2` 直接提到 `0.1.5`

### 错误 3：试图重新发布已经存在的 npm 版本

表现：

- npm 返回 `403 Forbidden`

典型报错：

```text
You cannot publish over the previously published versions: 0.1.2
```

处理方式：

- npm 已发布版本不可覆盖
- 必须发布一个全新的版本号

## 推荐命令模板

把 `<VERSION>` 替换成目标版本：

```bash
git fetch origin
git checkout main
git pull --ff-only origin main

# 手动修改 Cargo.toml version = "<VERSION>"

node scripts/npm/sync-packages.mjs
cargo test

git add Cargo.toml Cargo.lock npm/main/package.json npm/platforms/*/package.json
git commit -m "bump version <VERSION>"
git push origin main

git tag "v<VERSION>"
git push origin "refs/tags/v<VERSION>"

gh run list --workflow publish-npm --limit 5
```

## 本仓库当前规则

截至这份 SOP 编写时，必须记住这两个规则：

1. 发 npm 的唯一入口是 `publish-npm`
2. `publish-npm` 的唯一触发条件是 `v*` tag push

后续如果 workflow 触发条件改了，这份 SOP 也要一起更新。
