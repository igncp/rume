#!/bin/bash

set -e

if [ "$SKIP_SOURCE" != "1" ]; then
    . ../../../scripts/local_use_system_clang.sh
fi

echo "Compiling C test application 'test_rume'..."

astyle test_rume.c
rm -f test_rume.c.orig

clang test_rume.c \
    -L ../../target/release \
    -I ../../include \
    -lrume \
    -o test_rume_c

RUME_LOG_DIR=$PWD \
    ./test_rume_c
