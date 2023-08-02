#!/bin/zsh

ROOT_DIR=$(pwd)
API_DIR=$ROOT_DIR/mangadex-api-tsrs
BINDINGS_DIR=$ROOT_DIR/src/util/bindings

mkdir -p $BINDINGS_DIR

generate_bindings() {
    cd ${1:?"Path must be specified"}
    cargo test
    cp bindings/*.ts $BINDINGS_DIR/
}

generate_bindings $API_DIR/mangadex-api-schema
generate_bindings $API_DIR/mangadex-api-types
