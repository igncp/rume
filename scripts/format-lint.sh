#!/bin/bash

set -e

shfmt -w scripts

find src -name '*.rs' | xargs rustfmt
find tools -name '*.rs' | xargs rustfmt

astyle -n include/rume_api.h
astyle -n include/rume_extension.h
astyle -n test/rume_c/test_rume.c

echo "Rume: Format and lint checks passed."
