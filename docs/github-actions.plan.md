<!-- a1ebcf37-20a7-4700-977d-c4f2f9ca3bcb 97c3084b-5657-47b9-80f3-3eb5873f6c4c -->
# GitHub Actions 多平台自动构建部署

## 执行步骤

### 1. 创建 .gitignore 文件

在项目根目录创建 `.gitignore`，忽略构建产物和依赖：

- `node_modules/`, `dist/`, `src-tauri/target/`, `src-tauri/gen/`
- IDE 配置文件、macOS `.DS_Store`
- Android/iOS 构建产物

### 2. 创建 GitHub Actions Workflow

创建 `.github/workflows/release.yml`，配置多平台构建：

**关键配置**：

- **触发条件**：推送 `v*` 标签（如 v1.0.0）或手动触发
- **5 个并行 job**：
- `build-macos`：构建 macOS 桌面应用（.dmg）
- `build-windows`：构建 Windows 应用（.exe/.msi）
- `build-linux`：构建 Linux 应用（.deb/.AppImage）
- `build-android`：构建 Android APK（所有架构）
- `build-ios`：构建 iOS 应用（模拟器版本，未签名）
- **最终 job**：`create-release` 自动创建 GitHub Release 并上传所有构建产物

**使用包管理器**：pnpm（从 package.json 和 pnpm-lock.yaml 识别）

**构建命令**：

- 桌面端：`pnpm tauri build`
- Android：`pnpm tauri android build`
- iOS：`pnpm tauri ios build`

### 3. 初始化 Git 仓库并推送

执行步骤：

1. `git init` - 初始化 Git
2. `git add .` - 添加所有文件
3. `git commit -m "Initial commit: Multi-platform Tauri app"` - 首次提交
4. `git remote add origin https://github.com/RagnorLi/ragnor-tauri-app.git` - 关联远程仓库
5. `git branch -M main` - 重命名为 main 分支
6. `git push -u origin main` - 推送代码

### 4. 创建版本标签触发构建

1. `git tag v1.0.0` - 创建版本标签
2. `git push origin v1.0.0` - 推送标签，自动触发 GitHub Actions

### 5. 监控构建过程

访问 https://github.com/RagnorLi/ragnor-tauri-app/actions 查看：

- 5 个平台的并行构建状态
- 每个 job 的详细日志
- 预计总时间：15-20 分钟

### 6. 下载构建产物

构建完成后，在以下位置获取安装包：

- **Releases 页面**：https://github.com/RagnorLi/ragnor-tauri-app/releases/tag/v1.0.0
- **Artifacts**：workflow 运行详情页面底部

## 关键文件

**创建的文件**：

- `.gitignore` - Git 忽略规则
- `.github/workflows/release.yml` - GitHub Actions 配置

**使用的仓库**：

- https://github.com/RagnorLi/ragnor-tauri-app.git

## 注意事项

1. **网络要求**：GitHub Actions 构建需要下载依赖，首次构建时间较长
2. **iOS 限制**：未签名的构建只能生成模拟器版本，无法在真机运行
3. **Android APK**：生成的是 debug 版本，可直接在 Android 设备安装测试
4. **公开仓库**：已是公开仓库，Actions 免费无限制使用

## 后续优化（可选）

完成基础构建后，可进一步优化：

- 添加 Android keystore 实现签名 APK
- 配置 Apple Developer 证书生成可安装的 iOS IPA
- 添加自动版本号管理
- 配置缓存加速构建

### To-dos

- [ ] 创建 .gitignore 文件，忽略 node_modules、dist、target、gen 等构建产物
- [ ] 创建 .github/workflows/release.yml，配置 5 个平台的构建 job
- [ ] 初始化 Git 仓库并关联 GitHub 远程仓库
- [ ] 提交所有文件并推送到 GitHub main 分支
- [ ] 创建 v1.0.0 标签并推送，触发 GitHub Actions 构建
- [ ] 监控 GitHub Actions 构建过程，确保所有平台构建成功