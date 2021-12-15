#!/bin/bash

TARGET=wasm32-unknown-unknown
NAME=do_addition
BINARY=target/$TARGET/release/$NAME.wasm

cargo build --target $TARGET --release
