#!/bin/bash

# 将这个函数添加到你的 .bashrc 或 .zshrc 文件中
function qs() {
  local tmp_file
  tmp_file=$(mktemp)

  /home/debin/Codes/tools/quickswitch/quickswitch/target/release/quickswitch --output-file "$tmp_file"

  local dest_path
  dest_path=$(cat "$tmp_file")

  rm "$tmp_file"

  if [[ -n "$dest_path" && -d "$dest_path" ]]; then
    cd "$dest_path"
  else
    if [[ -n "$dest_path" ]]; then
        echo "错误: TUI 程序输出的路径无效: '$dest_path'" >&2
    fi
  fi
}