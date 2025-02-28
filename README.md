# minigrep 使用教程

## 简介
`minigrep` 是一个简单的命令行工具，用于在文件中搜索指定的文本模式。它可以帮助你快速定位包含特定字符串的行，适用于各种文本文件的搜索场景。

## 安装
在使用 `minigrep` 之前，你需要确保已经将其安装到你的系统中。具体的安装步骤可能因你的操作系统和开发环境而异。以下是一些常见的安装方式：

### 从源码编译安装
如果你有 `minigrep` 的源代码，可以按照以下步骤进行编译和安装：

1. 克隆或下载 `minigrep` 的源代码仓库到本地：
   ```sh
   git clone https://github.com/goudabuliu/minigrep
   cd minigrep
2. 编译代码
   ```sh
   cargo build --release
3. 将编译好的可执行文件移动到系统的可执行路径中
   ```sh
   sudo cp target/release/minigrep /usr/local/bin/

## 使用说明

### 基本用法
    minigrep <pattern> <filename>
### 不区分大小写
    IGNORE_CASE=1 minigrep pattern example.txt