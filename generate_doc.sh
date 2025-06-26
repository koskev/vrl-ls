#!/bin/bash -ex

TEMP_DIR=$(mktemp -d)

git clone https://github.com/vectordotdev/vector "$TEMP_DIR/vector"
pushd "$TEMP_DIR/vector" || exit
./scripts/cue.sh export | jq '.remap.functions' > functions.json
popd || exit
mv "$TEMP_DIR/vector/functions.json" ./
rm -rf "$TEMP_DIR"

