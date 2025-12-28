#!/bin/bash

set -e

echo "Compiling C test application 'test_rume'..."

clang test_rume.c \
    -L ../../target/release \
    -I ../../include \
    -lrume \
    -o test_rume_c

RUME_LOG_DIR=$PWD \
    ./test_rume_c
