#!/bin/bash
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SSH="sebastianwhiffen@192.168.0.148"
readonly TARGET_PATH="/home/pi/coffee_machine"
readonly SOURCE_PATH="./target/${TARGET_ARCH}/release/coffee_machine"

cargo build --release --target=${TARGET_ARCH}

read -sp "Enter SSH password: " PW
echo

sshpass -p "$PW" rsync \
  --rsync-path="sudo rsync" \
  -avz -e ssh \
  "$SOURCE_PATH" \
  "$SSH":"$TARGET_PATH"

sshpass -p "$PW" ssh -t ${SSH} ${TARGET_PATH}
