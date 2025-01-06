#!/usr/bin/env sh

set -e

rm -rf ./target/leancloud
mkdir ./target/leancloud
cp ./assets/leanengine.yaml ./target/leancloud
cp ./assets/waline.sqlite ./target/leancloud
cp ./target/x86_64-unknown-linux-musl/release/waline-mini ./target/leancloud

cd ./target/leancloud

git init
git add -A
git commit -m 'leancloud'
git checkout -b 'leancloud'
git push -f git@github.com:JQiue/waline-mini.git leancloud

cd -
