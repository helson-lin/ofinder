#!/bin/sh

# 获取当前目录下的可执行文件名
executable=finder

# 检查是否找到可执行文件
if [ -z "$executable" ]; then
  echo "No executable found in the current directory."
  exit 1
fi

# 获取 Fish Shell 配置文件路径
fish_config="$HOME/.config/fish/config.fish"

# 检查配置文件是否存在
if [ ! -f "$fish_config" ]; then
  echo "Fish Shell config file not found."
  exit 1
fi

# 添加 Rust 可执行文件到配置文件
echo "Adding '$executable' to Fish Shell config file..."
echo "$executable=\"$PWD/$executable\""
echo "alias $executable=\"$PWD/$executable\"" >> "$fish_config"

# 重新加载 Fish Shell 配置
fish -c "source $fish_config"

echo "Executable added successfully. You can now use '$executable' in Fish Shell."

