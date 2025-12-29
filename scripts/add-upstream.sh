set -e
git remote rm upstream || true
git remote add upstream https://github.com/rime/librime.git
git fetch upstream
