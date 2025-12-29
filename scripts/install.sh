#!/bin/bash

###############################################################
# 严格模式
set -euo pipefail # 兼容性写法（部分 shell 可能不支持 pipefail）
shopt -s failglob nullglob

# 调试模式（按需启用），用法：DEBUG=true ./your_script.sh
if [[ "${DEBUG:-}" == "true" ]]; then
  set -xv
fi

# 清理函数
cleanup() {
    echo "清理资源..."
}

# 清理和错误处理
trap 'echo "Error at line $LINENO"; cleanup' ERR
###############################################################

cd .. && pwd
cargo clean
#cargo build --release --bin git_commit_generator
cargo build --release --bins

# 安装所有编译的二进制工具到 ~/data/bin
INSTALL_DIR="$HOME/data/bin"
mkdir -p "$INSTALL_DIR"

echo "==> 安装二进制工具到 $INSTALL_DIR"

# 遍历 src/bin 目录下的所有 .rs 文件
for rs_file in src/bin/*.rs; do
    # 获取文件名（去掉路径和 .rs 后缀）
    bin_name=$(basename "$rs_file" .rs)

    # 源文件路径
    src_bin="./target/release/$bin_name"

    # 检查编译后的二进制文件是否存在
    if [[ -f "$src_bin" ]]; then
        echo -n "✅ 安装: $bin_name 路径: " && cp -fv "$src_bin" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/$bin_name"
    else
        echo "  警告: 未找到编译后的二进制文件: $src_bin"
    fi
done

echo "==> 安装完成！"