#!/bin/bash

###############################################################
# 严格模式
set -euo pipefail # 兼容性写法（部分 shell 可能不支持 pipefail）
shopt -s failglob nullglob

# 调试模式（按需启用），用法：DEBUG=true ./your_script.sh
if [[ "${DEBUG:-}" == "true" ]]; then
  set -xv
fi

# 清理和错误处理
trap 'echo "Error at line $LINENO"; cleanup' ERR
###############################################################

cd .. && pwd
cargo clean
cargo build --release --bin git_commit_generator
mv -fv ./target/release/git_commit_generator ~/data/bin