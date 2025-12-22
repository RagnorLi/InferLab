<!-- 7f21ca01-368b-4160-a2a1-19fd2106b73e fecb84b3-4608-4428-a205-761741d4dfaa -->
# Tauri 多平台 Hello World 应用构建实践（macOS + iOS + Android）

## 阶段一：本地开发环境准备

### 1. 安装必需工具链

#### 1.1 Node.js 和包管理器

```bash
# 检查 Node.js 版本
node --version  # 推荐 v18 LTS 或更高

# 检查包管理器（npm/pnpm/yarn 任选其一）
npm --version
# 或
pnpm --version
```

#### 1.2 Rust 工具链（重要：必须使用 rustup）

**警告**：不要使用 Homebrew 安装 Rust！

```bash
# ❌ 错误方式（会导致 Tauri 移动端构建失败）
brew install rust

# ✅ 正确方式：使用官方 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装后重启终端，验证
rustup --version
cargo --version
rustc --version
```

**原因**：

- Homebrew Rust 缺少 `rustup` 工具，无法管理交叉编译 target
- Tauri 移动端需要为 iOS/Android 添加多个编译目标
- 只有 rustup 能管理这些 target

**如果已安装 Homebrew Rust，先卸载**：

```bash
brew uninstall rust
# 然后再安装 rustup
```

#### 1.3 Xcode（macOS 和 iOS 必需）

```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 验证
xcode-select -p
```

#### 1.4 Android Studio 和 NDK

1. 下载并安装 [Android Studio](https://developer.android.com/studio)
2. 打开 Android Studio → Settings → SDK Manager
3. 安装以下组件：

- Android SDK Platform（推荐 API 33+）
- Android SDK Build-Tools
- NDK（通过 SDK Tools 标签页）
- Android SDK Command-line Tool

4. 配置环境变量（添加到 `~/.zshrc`）：
```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"
export PATH="$ANDROID_HOME/platform-tools:$PATH"
```

5. 重启终端，验证：
```bash
echo $ANDROID_HOME
adb --version
```


### 2. 创建 Tauri + React 项目

使用官方脚手架创建项目：

```bash
pnpm create tauri-app@latest
```

选择：

- 项目名称：hello-world-tauri
- 前端框架：React
- 包管理器：npm/yarn/pnpm

```shell
ragnor@WhyNotMe-2 app % pnpm create tauri-app@latest
.../19ae6f803ac-3d34                     |   +3 +
.../19ae6f803ac-3d34                     | Progress: resolved 12, reused 3, downloaded 0, added 3, done
✔ Project name · infer-lab
✔ Identifier · com.ragnor.infer-lab
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)
✔ Choose your package manager · pnpm
✔ Choose your UI template · React - (https://react.dev/)
✔ Choose your UI flavor · TypeScript

Template created! To get started run:
  cd infer-lab
  pnpm install
  pnpm tauri android init
  pnpm tauri ios init

For Desktop development, run:
  pnpm tauri dev

For Android development, run:
  pnpm tauri android dev

For iOS development, run:
  pnpm tauri ios dev

```

### 3. 初始化移动端支持

```bash
pnpm install
pnpm run tauri init
pnpm run tauri android init
pnpm run tauri ios init
```

## 阶段二：本地构建验证

### 4. 桌面端本地构建（macOS）

```bash
pnpm run tauri build
```

验证生成的 `.app` 和 `.dmg` 文件

### 5. iOS 本地构建

#### 5.1 环境验证

```bash
# 检查 Xcode 命令行工具
xcode-select -p

# 检查 iOS Rust targets
rustup target list --installed | grep ios
# 应包含：
# - aarch64-apple-ios        (真机 ARM64)
# - aarch64-apple-ios-sim    (M1/M2 Mac 模拟器)
# - x86_64-apple-ios         (Intel Mac 模拟器)
```

如果缺少 targets：

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

#### 5.2 安装 CocoaPods

iOS 项目依赖 CocoaPods 管理原生依赖：

```bash
# 方式 1：通过 Homebrew 安装（推荐）
brew install cocoapods

# 方式 2：通过 gem 安装
sudo gem install cocoapods

# 验证安装
pod --version
```

#### 5.3 开发模式

```bash
# 初始化 iOS 项目（首次）
pnpm run tauri ios init

# 开发模式（自动打开模拟器）
pnpm run tauri ios dev
```

**在 Xcode 中打开**：

- 项目路径：`src-tauri/gen/apple/ragnor-tauri-app.xcodeproj`
- 选择模拟器或真机设备
- 点击运行按钮（⌘+R）

#### 5.4 常见问题

**问题：CocoaPods 安装失败**

```
Error: Failed to install `cocoapods`: No such file or directory
```

解决：

```bash
brew install cocoapods
pnpm run tauri ios init  # 重新初始化
```

**问题：代码签名证书警告**

```
Warn: No code signing certificates found.
```

- 模拟器测试：可忽略，不影响
- 真机调试：需要 Apple ID（免费，每 7 天重签）
- 发布：需要 Apple Developer Program ($99/年)

### 6. Android 本地构建

#### 6.1 环境验证（首次必做）

```bash
# 检查 Android 环境变量
echo $ANDROID_HOME  
# 应输出：/Users/你的用户名/Library/Android/sdk

# 检查 Rust Android targets（必需）
rustup target list --installed | grep android
# 应包含以下 4 个架构：
# - aarch64-linux-android      (64位 ARM - 现代手机)
# - armv7-linux-androideabi    (32位 ARM - 老设备)
# - i686-linux-android         (32位 x86 - 模拟器)
# - x86_64-linux-android       (64位 x86 - 模拟器)
```

如果缺少 targets，执行：

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

#### 6.2 开发模式（推荐使用 Android Studio）

**方式 1：Android Studio（推荐）**

1. 打开 Android Studio，选择 "Open" → 打开 `src-tauri/gen/android/` 目录
2. 等待 Gradle 同步完成（首次需下载依赖 ~550MB，约 10-30 分钟）
3. **解决架构不匹配错误**：
   ```
   常见错误：
   The currently selected variant 'armDebug' uses split APKs, 
   but none of the 1 split apks are compatible with the current 
   device with ABIs 'arm64-v8a'
   
   解决方案：
   - 打开 View → Tool Windows → Build Variants
   - 在 Module "app" 的下拉菜单中选择：
     * arm64Debug（64位设备，推荐）
     * 或 universalDebug（兼容所有架构）
   - 等待 Gradle 同步完成
   ```

4. 点击绿色运行按钮，选择设备/模拟器

**方式 2：命令行**

```bash
pnpm run tauri android dev
```

#### 6.3 生产构建（生成 APK）

```bash
# 默认构建（生成所有架构的 APK）
pnpm run tauri android build

# 只构建 64 位 ARM（现代设备，最常用）
pnpm run tauri android build -- --target aarch64-linux-android

# 只构建 32 位 ARM（老设备兼容）
pnpm run tauri android build -- --target armv7-linux-androideabi

# 生成 AAB（用于 Google Play 上传）
pnpm run tauri android build -- --aab

# 查看所有可用选项
pnpm run tauri android build -- --help
```

**注意**：`--apk` 参数需要值（true/false），默认已生成 APK，无需指定。

#### 6.4 构建产物位置

APK 文件位置：

- Universal（所有架构）：`src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk`
- ARM64（64位）：`src-tauri/gen/android/app/build/outputs/apk/arm64/debug/app-arm64-debug.apk`
- ARM32（32位）：`src-tauri/gen/android/app/build/outputs/apk/armeabi-v7a/debug/app-armeabi-v7a-debug.apk`

#### 6.5 常见问题

**问题 1：NDK 安装失败**

```
Error: Failed to install Android NDK
```

解决：通过 Android Studio 的 SDK Manager 手动安装 NDK

**问题 2：rustup 命令未找到**

```
Error: No such file or directory (os error 2)
```

原因：使用 Homebrew 安装的 Rust 缺少 rustup

解决：卸载 Homebrew Rust，安装官方 rustup：

```bash
brew uninstall rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**问题 3：Gradle 下载慢**

解决：配置国内镜像（创建 `~/.gradle/init.gradle`）

## 阶段三：GitHub Actions 自动化部署

### 7. 准备工作：初始化 Git 仓库

#### 7.1 创建 GitHub 仓库

1. 访问 https://github.com/new
2. 创建仓库（**推荐公开仓库**，免费无限 Actions 分钟数）

- 仓库名：`tauri-hello-world`（或你的项目名）
- 可见性：Public
- **不要**勾选 "Add a README file"（本地已有）

#### 7.2 初始化本地 Git 仓库

```bash
# 进入项目根目录
cd /Users/ragnor/Coder/Project/AiProj/CoffeeAI/tauri/ragnor-tauri-app

# 初始化 Git（如果还没有）
git init

# 添加所有文件
git add .

# 首次提交
git commit -m "Initial commit: Tauri Hello World app"

# 关联远程仓库（替换为你的 GitHub 用户名和仓库名）
git remote add origin https://github.com/你的用户名/tauri-hello-world.git

# 推送代码
git branch -M main
git push -u origin main
```

**⚠️ 实际遇到的问题**：

```
error: failed to push some refs to 'https://github.com/RagnorLi/ragnor-tauri-app.git'
hint: Updates were rejected because the remote contains work that you do not have locally.
```

**原因**：GitHub 仓库创建时勾选了 "Add a LICENSE file"，导致远程有本地没有的 commit（历史不相关）。

**解决方案**：

```bash
# 拉取远程内容并合并不相关的历史
git pull origin main --allow-unrelated-histories

# 重新推送
git push -u origin main
```

**知识点**：

- `--allow-unrelated-histories`：允许合并两个没有共同祖先的Git历史
- `-u` = `--set-upstream`：建立本地分支与远程分支的跟踪关系
- 只需在首次推送时使用 `-u`，之后可以直接用 `git push`
````

#### 7.3 补充 .gitignore 规则

**实际情况**：项目根目录已有基础 `.gitignore`，但缺少 Tauri 相关规则。

需要在现有 `.gitignore` 末尾添加：

```gitignore
# Tauri
src-tauri/target/
src-tauri/gen/

# Android
*.apk
*.aab
local.properties

# iOS
*.ipa
Pods/
````


**原因**：`src-tauri/gen/` 目录包含生成的 Android/iOS 项目文件，不应提交到 Git。

### 8. 创建 GitHub Actions Workflow

#### 8.1 创建工作流文件

创建文件结构：

```bash
mkdir -p .github/workflows
```

#### 8.2 创建多平台构建配置

创建 `.github/workflows/release.yml`：

**⚠️ 重要**：必须在文件开头添加权限配置，否则创建 Release 会失败！

```yaml
name: Release Multi-Platform Builds

permissions:
  contents: write  # 允许创建 Release

on:
  push:
    tags:
      - 'v*'  # 当推送 v* 格式的标签时触发（如 v1.0.0）
  workflow_dispatch:  # 允许在 GitHub 网页手动触发

jobs:
  # ==================== macOS 桌面端 ====================
  build-macos:
    runs-on: macos-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies
        run: pnpm install

      - name: Build macOS app
        run: pnpm tauri build

      - name: Upload macOS artifacts
        uses: actions/upload-artifact@v4
        with:
          name: macos-app
          path: |
            src-tauri/target/release/bundle/dmg/*.dmg
            src-tauri/target/release/bundle/macos/*.app.tar.gz

  # ==================== Windows 桌面端 ====================
  build-windows:
    runs-on: windows-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies
        run: pnpm install

      - name: Build Windows app
        run: pnpm tauri build

      - name: Upload Windows artifacts
        uses: actions/upload-artifact@v4
        with:
          name: windows-app
          path: |
            src-tauri/target/release/bundle/nsis/*.exe
            src-tauri/target/release/bundle/msi/*.msi

  # ==================== Linux 桌面端 ====================
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install system dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            libgtk-3-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev

      - name: Install dependencies
        run: pnpm install

      - name: Build Linux app
        run: pnpm tauri build

      - name: Upload Linux artifacts
        uses: actions/upload-artifact@v4
        with:
          name: linux-app
          path: |
            src-tauri/target/release/bundle/deb/*.deb
            src-tauri/target/release/bundle/appimage/*.AppImage

  # ==================== Android ====================
  build-android:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Android targets
        run: |
          rustup target add aarch64-linux-android
          rustup target add armv7-linux-androideabi
          rustup target add i686-linux-android
          rustup target add x86_64-linux-android

      - name: Setup Java
        uses: actions/setup-java@v4
        with:
          distribution: 'temurin'
          java-version: '17'

      - name: Setup Android SDK
        uses: android-actions/setup-android@v3

      - name: Install Android NDK
        run: |
          sdkmanager "ndk;25.1.8937393"
          echo "ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.1.8937393" >> $GITHUB_ENV

      - name: Install dependencies
        run: pnpm install
      
      - name: Initialize Android project
        run: pnpm tauri android init

      - name: Build Android APK
        run: pnpm tauri android build

      - name: Upload Android artifacts
        uses: actions/upload-artifact@v4
        with:
          name: android-apk
          path: src-tauri/gen/android/app/build/outputs/apk/**/*.apk

  # ==================== iOS ====================
  build-ios:
    runs-on: macos-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install iOS targets
        run: |
          rustup target add aarch64-apple-ios
          rustup target add aarch64-apple-ios-sim
          rustup target add x86_64-apple-ios

      - name: Install CocoaPods
        run: sudo gem install cocoapods

      - name: Install dependencies
        run: pnpm install

      - name: Build iOS app
        run: pnpm tauri ios build
        continue-on-error: true  # iOS 构建可能因签名问题失败，但不影响其他平台

      - name: Upload iOS artifacts
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: ios-app
          path: src-tauri/gen/apple/build/**/*.app

  # ==================== 创建 GitHub Release ====================
  create-release:
    needs: [build-macos, build-windows, build-linux, build-android, build-ios]
    runs-on: ubuntu-latest
    if: always()  # 即使某个平台失败也创建 release
    steps:
      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            artifacts/**/*
          draft: false
          prerelease: false
          generate_release_notes: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```


### 10. 配置代码签名（可选，生产环境推荐）

#### 10.1 Android 签名配置

**生成 keystore**：

```bash
# 在本地生成签名密钥
keytool -genkey -v -keystore release-key.keystore \
  -alias infer-lab-alias \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000

# 执行命令后会交互式提示输入 keystore 密码
# 注意：从 Java 9 开始，keytool 默认使用 PKCS12 格式
# 在 PKCS12 格式中，keystore 密码和密钥密码必须是相同的
# 所以 keytool 只会提示输入一次密码，密钥密码会自动使用相同值
# 
# 重要：请务必记住这个密码，后续需要配置到 GitHub Secrets！
```

**配置 GitHub Secrets**：

1. 转换 keystore 为 base64：
   ```bash
   base64 release-key.keystore > release-key.base64

   or mac

   base64 -i release-key.keystore -o release-key.base64
   ```

2. 在 GitHub 仓库页面：

- 进入 Settings → Secrets and variables → Actions
- 点击 "New repository secret"
- 添加以下 secrets：
  - `ANDROID_KEYSTORE_BASE64`：上面生成的 base64 内容（整个文件内容）
  - `ANDROID_KEYSTORE_PASSWORD`：keystore 密码（生成时输入的密码）
  - `ANDROID_KEY_ALIAS`：别名（如 `infer-lab-alias`，与生成命令中的 `-alias` 参数一致）
  - `ANDROID_KEY_PASSWORD`：密钥密码（与 keystore 密码相同，因为 PKCS12 格式要求两者必须一致）

**在 workflow 中使用**：

在 Android 构建步骤前添加：

```yaml
- name: Decode keystore
  run: |
    echo "${{ secrets.ANDROID_KEYSTORE_BASE64 }}" | base64 -d > release-key.keystore

- name: Build signed APK
  env:
    KEYSTORE_PATH: release-key.keystore
    KEYSTORE_PASSWORD: ${{ secrets.ANDROID_KEYSTORE_PASSWORD }}
    KEY_ALIAS: ${{ secrets.ANDROID_KEY_ALIAS }}
    KEY_PASSWORD: ${{ secrets.ANDROID_KEY_PASSWORD }}
  run: npm run tauri android build -- --release
```

#### 10.2 iOS 签名配置（需要 Apple Developer 账号）

**准备证书**：

1. 在 Apple Developer 网站创建证书和 Provisioning Profile
2. 导出 `.p12` 证书文件
3. 转换为 base64 并添加到 GitHub Secrets

**配置 GitHub Secrets**：

- `APPLE_CERTIFICATE_BASE64`：证书 base64
- `APPLE_CERTIFICATE_PASSWORD`：证书密码
- `PROVISIONING_PROFILE_BASE64`：配置文件 base64

#### 10.3 macOS 签名配置

类似 iOS，需要 Apple Developer 证书进行公证（notarization）。

### 11. 提交 Workflow 并推送

```bash
# 添加 workflow 文件
git add .github/workflows/

# 提交
git commit -m "Add GitHub Actions workflow for multi-platform builds"

# 推送
git push origin main
```

### 12. 触发构建

#### 方式 1：通过标签触发（推荐用于发布）

```bash
# 创建版本标签
git tag v1.0.0

# 推送标签到远程（触发 release.yml）
git push origin v1.0.0
```

#### 方式 2：手动触发

1. 访问 GitHub 仓库页面
2. 进入 Actions 标签页
3. 选择你的 workflow
4. 点击 "Run workflow" 按钮
5. 选择分支并确认

#### 方式 3：推送代码触发（如果配置了 on: push）

```bash
# 直接推送代码即可触发
git push origin main
```

### 13. 首次构建：预期会失败

**实际经验**：首次推送标签后触发的构建会部分失败，这是正常的。

访问 https://github.com/RagnorLi/ragnor-tauri-app/actions 查看构建状态。

**实际构建结果**：

| Job | 状态 | 耗时 | 错误原因 |
|-----|------|------|---------|
| build-macos | ✅ 成功 | ~4分钟 | - |
| build-windows | ✅ 成功 | ~6分钟 | - |
| build-linux | ❌ 失败 | ~24秒 | `Unable to locate package libwebkit2gtk-4.0-dev` |
| build-android | ❌ 失败 | ~1分钟 | `Android Studio project directory doesn't exist` |
| build-ios | ✅ 成功 | ~33秒 | - |
| create-release | ❌ 失败 | ~7秒 | `GitHub release failed with status: 403` |

#### 失败原因分析

**1. build-linux 失败**

错误信息：

```
E: Unable to locate package libwebkit2gtk-4.0-dev
E: Couldn't find any package by glob 'libwebkit2gtk-4.0-dev'
```

根本原因：Ubuntu 24.04 的包名已更新

- 旧包名：`libwebkit2gtk-4.0-dev`（Ubuntu 22.04）
- 新包名：`libwebkit2gtk-4.1-dev`（Ubuntu 24.04）

**2. build-android 失败**

错误信息：

```
Android Studio project directory /home/runner/work/.../src-tauri/gen/android doesn't exist.
Please run 'tauri android init' and try again.
```

根本原因：`src-tauri/gen/` 目录被 .gitignore 忽略

- 本地有该目录（本地构建生成的）
- Git 推送时被忽略
- CI 环境中没有该目录

**3. create-release 失败**

错误信息：

```
GitHub release failed with status: 403
Too many retries. Aborting...
```

根本原因：workflow 缺少创建 Release 的权限配置

### 14. 修复 workflow 配置

编辑 `.github/workflows/release.yml`，进行以下 3 处修复：

**修复 1：添加权限配置（第 3-4 行）**

```yaml
permissions:
  contents: write  # 允许创建 GitHub Release
```

**修复 2：更新 Linux 包名（第 587 行）**

```yaml
libwebkit2gtk-4.0-dev    # 改为 ↓
libwebkit2gtk-4.1-dev
```

**修复 3：添加 Android 初始化步骤（第 168-169 行）**

在 `Install dependencies` 之后添加：

```yaml
- name: Initialize Android project
  run: pnpm tauri android init
```

### 15. 删除旧标签并重新触发构建

修复后需要删除旧标签并重新创建，以触发新的构建：

```bash
# 1. 提交修复
git add .
git commit -m "Fix CI workflow: update Linux deps and add Android init"

# 2. 推送修复到 main 分支
git push origin main

# 3. 删除本地旧标签
git tag -d v1.0.0

# 4. 删除远程旧标签
git push origin :refs/tags/v1.0.0

# 5. 重新创建标签（指向修复后的 commit）
git tag v1.0.0

# 6. 推送新标签（重新触发 workflow）
git push origin v1.0.0
```

**关键知识**：

- Git 标签是指向特定 commit 的指针，无法直接移动
- 必须先删除旧标签，才能重新创建指向新 commit 的标签
- `git push origin :refs/tags/v1.0.0` 是删除远程标签的语法

**简化方案**：如果不想删除旧标签，可以直接用新版本号：

```bash
git add .
git commit -m "Fix CI workflow"
git push origin main
git tag v1.0.1      # 使用新版本号
git push origin v1.0.1
```

### 16. 监控最终构建（全部成功）

访问 GitHub Actions 查看第二次构建：

**最终构建结果**：

| Job | 状态 | 实际耗时 |
|-----|------|---------|
| build-macos | ✅ 成功 | 4分 41秒 |
| build-windows | ✅ 成功 | 6分 42秒 |
| build-linux | ✅ 成功 | 24秒 |
| build-android | ✅ 成功 | 1分 1秒 |
| build-ios | ✅ 成功 | 33秒 |
| create-release | ✅ 成功 | 7秒 |

**总耗时**：约 6-7 分钟（并行执行）

**关键指标说明**：

- ✅ 绿色勾：成功
- ❌ 红色叉：失败（点击查看错误日志）
- 🔵 蓝色圆圈：正在运行

### 17. 下载和验证构建产物

#### 方式 1：从 Artifacts 下载（开发测试）

1. 在 workflow 运行详情页面
2. 滚动到底部的 "Artifacts" 区域
3. 点击下载对应平台的构建产物
4. 解压并测试

**注意**：Artifacts 有 90 天保存期限

#### 方式 2：从 Releases 下载（生产发布）

如果使用了 `create-release` job：

1. 访问仓库的 **Releases** 页面
2. 找到对应版本的 release
3. 下载 Assets 中的安装包

**生成的文件**：

- macOS：`AppName-1.0.0.dmg`
- Windows：`AppName_1.0.0_x64-setup.exe`
- Linux：`app-name_1.0.0_amd64.deb`、`.AppImage`
- Android：`app-universal-debug.apk`
- iOS：`AppName.app`（需签名才能生成 .ipa）

### 18. 实际测试各平台安装包

**macOS**：

```bash
# 打开 DMG
open AppName-1.0.0.dmg

# 或直接运行 .app
open AppName.app
```

**Windows**：

- 双击 `.exe` 安装器

**Linux**：

```bash
# Debian/Ubuntu
sudo dpkg -i app-name_1.0.0_amd64.deb

# AppImage
chmod +x AppName.AppImage
./AppName.AppImage
```

**Android**：

```bash
# 通过 adb 安装
adb install app-universal-debug.apk

# 或直接传输到手机点击安装
```

**iOS**：

- 需要签名后通过 TestFlight 或直接安装

## 阶段四：故障排除总结

### 常见问题速查表

| 错误信息 | 原因 | 解决方案 |
|---------|------|---------|
| `rustup target add: No such file or directory` | 使用 Homebrew 安装的 Rust | 卸载 Homebrew Rust，安装 rustup |
| `Failed to install Android NDK` | sdkmanager 无法下载/网络问题 | 通过 Android Studio GUI 手动安装 NDK |
| `Failed to install cocoapods` | CocoaPods 未安装 | `brew install cocoapods` |
| `armDebug uses split APKs, not compatible with arm64-v8a` | Build Variant 选择错误 | 切换到 arm64Debug 或 universalDebug |
| `ANDROID_HOME not set` | 环境变量未配置 | 添加到 `~/.zshrc` 并重启终端 |
| `Gradle downloading slow` | 国内网络访问 Google 慢 | 配置 Gradle 镜像 |
| `failed to push: Updates were rejected` | Git 历史冲突（远程有 LICENSE） | `git pull origin main --allow-unrelated-histories` |
| `Unable to locate package libwebkit2gtk-4.0-dev` | Ubuntu 24.04 包名已更新 | 使用 `libwebkit2gtk-4.1-dev` |
| `Android project directory doesn't exist` | gen/ 目录被 gitignore | workflow 中添加 `tauri android init` |
| `GitHub release failed with status: 403` | workflow 缺少权限 | 添加 `permissions: contents: write` |

### 环境检查清单

构建前执行以下检查，确保环境完整：

```bash
# === Node.js 和包管理器 ===
node --version       # 应显示 v18+ 
npm --version        # 或 pnpm --version

# === Rust 工具链 ===
rustup --version     # 必须存在！
cargo --version
rustc --version

# === iOS Rust targets ===
rustup target list --installed | grep ios
# 应包含：aarch64-apple-ios, aarch64-apple-ios-sim, x86_64-apple-ios

# === Android Rust targets ===
rustup target list --installed | grep android
# 应包含：aarch64-linux-android, armv7-linux-androideabi, 
#        i686-linux-android, x86_64-linux-android

# === Xcode（iOS/macOS 必需）===
xcode-select -p      # 应输出 Xcode 路径

# === CocoaPods（iOS 必需）===
pod --version        # 应显示版本号

# === Android 环境 ===
echo $ANDROID_HOME   # 应输出 SDK 路径
adb --version        # Android 调试工具
```

如果以上任何一项失败，回到对应章节重新安装。

## 关键技术点

**Tauri 工作原理：**

- 前端：React（编译为 HTML/CSS/JS）
- 后端：Rust 二进制
- WebView：各平台原生 WebView（WKWebView、WebView2、WebKitGTK）

**跨平台限制：**

- iOS 构建需要 macOS 环境 + Xcode
- Android 可在任意平台构建（但 macOS 最方便）
- 移动端发布需要对应开发者账号（Apple $99/年，Android 免费）

**GitHub Actions 资源消耗：**

- macOS runner：10x 计费倍率（公开仓库免费）
- Windows runner：2x 计费倍率
- Linux runner：1x 计费倍率

**关键区别：Homebrew Rust vs rustup**

| 特性 | Homebrew Rust | rustup Rust |
|------|---------------|-------------|
| 安装方式 | `brew install rust` | `sh.rustup.rs` |
| 版本管理 | 单一版本 | 多版本、多工具链 |
| 交叉编译 | ❌ 不支持 | ✅ 支持 |
| Tauri 移动端 | ❌ 无法构建 | ✅ 完全支持 |
| 推荐度 | ❌ 不推荐 | ✅ 必须使用 |

## 实战经验总结

### 成功部署时间线

1. **环境准备**：~2小时（首次安装所有工具）
2. **本地开发**：~1小时（创建项目、本地构建测试）
3. **CI/CD配置**：~30分钟（创建workflow、Git推送）
4. **首次构建失败**：~7分钟（发现3个问题）
5. **修复并重新构建**：~7分钟（全部成功）
6. **总计**：约 4 小时（包含学习和调试时间）

### 关键经验

#### 1. Git 操作

**学到的命令**：

```bash
# 合并不相关的历史
git pull origin main --allow-unrelated-histories

# 删除远程标签
git push origin :refs/tags/v1.0.0

# 查看上游分支
git branch -vv
```

**最佳实践**：

- 创建 GitHub 仓库时不要勾选任何文件（README、LICENSE）
- 或者在创建仓库后立即 clone 再开发
- 标签修复时用新版本号更简单（v1.0.1）

#### 2. GitHub Actions workflow

**必须注意的 3 点**：

1. 文件开头添加 `permissions: contents: write`
2. Linux 使用 `libwebkit2gtk-4.1-dev`（不是 4.0）
3. Android 构建前必须先 `tauri android init`

**调试技巧**：

- 点击失败的 job 查看详细日志
- 搜索 "Error:" 快速定位问题
- GitHub Actions 的错误信息很准确，仔细阅读

#### 3. .gitignore 规则

**Tauri 项目必须忽略**：

- `src-tauri/target/`（Rust 编译产物）
- `src-tauri/gen/`（移动端生成的项目文件）

**为什么 gen/ 要忽略**：

- 这些是自动生成的文件
- 不同环境可能不兼容
- CI 中会重新生成

#### 4. 构建时间优化

**实际耗时（并行）**：

- macOS：最慢（4-6分钟）
- Windows：第二（6-7分钟）
- Linux：最快（24秒）
- Android：1分钟
- iOS：33秒

**总并行时间**：取决于最慢的 job（~7分钟）

### 常见误区

❌ **误区 1**：以为第一次构建会成功

- ✅ **现实**：几乎必然会失败，这是正常的

❌ **误区 2**：用 Homebrew 安装 Rust

- ✅ **正确**：必须使用 rustup（移动端必需）

❌ **误区 3**：直接 `git push v1.0.0`

- ✅ **正确**：`git push origin v1.0.0`（指定远程仓库）

❌ **误区 4**：修改代码后直接推送标签

- ✅ **正确**：必须先删除旧标签或用新版本号

### 下一步建议

完成基础部署后，可以进一步优化：

1. **添加缓存加速构建**
   ```yaml
   - uses: actions/cache@v4
     with:
       path: ~/.cargo
       key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
   ```

2. **配置 Android 签名**

   - 生成 release keystore
   - 添加到 GitHub Secrets
   - 修改构建命令为 release 模式

3. **配置 iOS 签名**

   - 需要 Apple Developer 账号（$99/年）
   - 创建证书和 Provisioning Profile
   - 配置 fastlane 自动签名

4. **自动版本号管理**

   - 使用 `tauri-action` 自动从标签提取版本号
   - 或使用 semantic-release 自动化版本管理

5. **添加自动化测试**

   - 在构建前运行单元测试
   - 使用 Tauri 的 WebDriver 进行 E2E 测试

### 资源链接

- **Tauri 官方文档**：https://tauri.app/
- **GitHub Actions 文档**：https://docs.github.com/actions
- **你的成功案例**：https://github.com/RagnorLi/ragnor-tauri-app
- **Tauri Actions Marketplace**：https://github.com/marketplace?query=tauri

---

**祝贺！** 🎉 你已经成功完成了 Tauri 多平台应用的完整 CI/CD 部署流程。

### To-dos

- [x] 安装本地开发环境：Node.js、Rust、Xcode、Android Studio
- [x] 使用 create-tauri-app 创建 React 项目
- [x] 初始化移动端支持（iOS 和 Android）
- [x] 本地构建并测试 macOS 桌面应用
- [x] 本地测试 iOS 和 Android 构建
- [x] 创建 GitHub 公开仓库（https://github.com/RagnorLi/ragnor-tauri-app）
- [x] 解决 Git 推送冲突（远程 LICENSE 文件）
- [x] 补充 .gitignore 的 Tauri 相关规则
- [x] 创建 GitHub Actions workflow（5 平台构建）
- [x] 推送代码并创建 v1.0.0 标签
- [x] 分析首次构建失败（Linux/Android/create-release）
- [x] 修复 workflow 配置（3 处修复）
- [x] 删除旧标签并重新触发构建
- [x] 验证所有平台构建成功
- [x] 从 GitHub Releases 下载各平台安装包
- [ ] 配置代码签名证书（可选，用于生产发布）