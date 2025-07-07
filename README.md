# QuickSwitch

一个基于 Rust 的快速终端文件浏览器，提供美观的文本用户界面（TUI），让你能够快速浏览和选择文件夹。

## 功能特性

- 🚀 **快速导航**: 通过直观的 TUI 界面快速浏览文件系统
- 🔍 **实时搜索**: 支持实时过滤文件和文件夹
- 📁 **目录预览**: 右侧面板显示选中目录的内容或文件信息
- 📄 **文件内容预览**: 支持文本文件内容预览（前 100 行），显示行号
- 🔧 **Shell 集成**: 提供 bash 和 fish shell 函数，实现快速目录切换
- 🎨 **美观界面**: 使用 ratatui 构建的现代化终端界面
- ⚡ **高性能**: 异步处理，响应迅速

## 安装要求

- Rust 1.70+ (使用 2024 edition)
- 支持的操作系统: Linux, macOS, Windows

## 编译安装

### 1. 克隆项目

```bash
git clone <repository-url>
cd quickswitch
```

### 2. 编译项目

#### 开发版本

```bash
cargo build
```

#### 发布版本（推荐）

```bash
cargo build --release
```

编译完成后，可执行文件位于：

- 开发版本：`target/debug/quickswitch`
- 发布版本：`target/release/quickswitch`

### 3. 安装到系统（可选）

```bash
# 安装到 ~/.cargo/bin
cargo install --path .

# 或者手动复制到系统路径
sudo cp target/release/quickswitch /usr/local/bin/
```

## 使用方法

### Shell 函数集成（推荐）

为了实现快速目录切换功能，项目提供了 shell 函数包装器。

#### Bash/Zsh 用户

1. 将以下函数添加到你的 `~/.bashrc` 或 `~/.zshrc` 文件中：

```bash
eval "$(quickswitch --init bash)"

# 绑定到 Ctrl+Alt+E
bind -x '"\C-\M-E": qs'
```

2. 重新加载配置：

```bash
source ~/.bashrc  # 或 source ~/.zshrc
```

#### Fish 用户

1. 将以下函数添加到你的 `~/.config/fish/config.fish` 文件中：

```fish
quickswitch --init fish | source

# 绑定按键（可选/推荐）Ctrl + E
bind \ce qs
```

2. 重新加载配置：

```fish
source ~/.config/fish/config.fish
```

### 使用预制脚本

项目已经提供了预制的 shell 脚本，你只需要修改其中的路径并自行增加按键绑定：

```bash
# 复制并编辑 bash 脚本
cp quickswitch.sh ~/.local/bin/qs_setup.sh
# 编辑文件，修改 quickswitch 的路径
# 然后在 .bashrc 中添加: source ~/.local/bin/qs_setup.sh

# 或者复制并编辑 fish 脚本
cp quickswitch.fish ~/.config/fish/functions/qs.fish
# 编辑文件，修改 quickswitch 的路径
```

## 快捷键操作

| 快捷键      | 功能                      |
| ----------- | ------------------------- |
| `↑/↓`       | 上下选择文件/文件夹       |
| `←/→`       | 进入父目录/进入选中的目录 |
| `Enter`     | 选择当前目录并退出程序    |
| `Esc`       | 退出程序                  |
| `字符输入`  | 实时搜索过滤              |
| `Backspace` | 删除搜索字符              |

## 界面说明

```
┌─ Search files (ESC to quit, Enter to exit & cd, ←→ navigate, ↑↓ select) ─┐
│ your_search_term                                                          │
└──────────────────────────────────────────────────────────────────────────┘
┌─ Files - /current/path (filtered/total) ─┐ ┌─ Preview ─────────────────┐
│ 📁 directory1                            │ │ 📁 directory1             │
│ 📁 directory2                            │ │ 📄 file1.txt              │
│ 📄 file1.txt                             │ │ 📄 file2.md               │
│ 📄 file2.md                              │ │ ... and 5 more items     │
└──────────────────────────────────────────┘ └───────────────────────────┘
```

- **左侧面板**: 显示当前目录的文件和文件夹列表
- **右侧面板**: 显示选中项的预览
  - 对于文件夹：显示其内容列表
  - 对于文本文件：显示文件内容（带行号）
  - 对于二进制文件：显示文件大小信息

## 项目结构

```
quickswitch/
├── Cargo.toml          # 项目配置和依赖
├── Cargo.lock          # 依赖锁定文件
├── src/
│   ├── main.rs         # 主程序文件
│   └── tui.rs          # TUI 相关代码（如果存在）
├── quickswitch.sh      # Bash/Zsh 集成脚本
├── quickswitch.fish    # Fish shell 集成脚本
└── target/             # 编译输出目录
```

## 依赖说明

主要依赖库：

- `ratatui`: 现代化的 Rust TUI 库
- `crossterm`: 跨平台终端操作库
- `anyhow`: 错误处理库
- `tokio`: 异步运行时

## 故障排除

### 编译问题

1. **Rust 版本过旧**

   ```bash
   # 更新 Rust
   rustup update
   ```

2. **依赖下载失败**
   ```bash
   # 清理并重新构建
   cargo clean
   cargo build --release
   ```

### 运行问题

1. **程序无法启动**

   - 确保你在终端环境中运行
   - 检查文件权限：`chmod +x target/release/quickswitch`

2. **界面显示异常**

   - 确保终端支持 UTF-8
   - 尝试调整终端窗口大小

3. **Shell 函数不工作**
   - 确保修改了脚本中的 quickswitch 路径
   - 确保重新加载了 shell 配置
   - 检查可执行文件权限

## 开发贡献

欢迎提交 issues 和 pull requests！

### 开发环境设置

```bash
# 克隆项目
git clone <repository-url>
cd quickswitch

# 安装开发依赖
cargo build

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 许可证

本项目采用 [许可证名称] 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 更新日志

### v0.1.0

- 初始版本
- 基本的文件浏览功能
- 实时搜索过滤
- 文件内容预览
- Shell 集成支持
