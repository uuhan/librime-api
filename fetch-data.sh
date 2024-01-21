#!/bin/bash

DIR=$(dirname "$0")

pushd "$DIR" &>/dev/null || exit

git clone https://github.com/iDvel/rime-ice rime-ice

popd &>/dev/null || return
