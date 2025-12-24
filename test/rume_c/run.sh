#!/bin/bash

set -e

if [ "$SKIP_SOURCE" != "1" ]; then
    . ../../../scripts/local_use_system_clang.sh
fi

echo "Compiling C test application 'rume_c'..."

clang main.c \
    -L ../../target/release \
    -I ../../include \
    -lrume \
    -o c_app

./c_app
