#!/bin/bash

# Create ./bin directory if it does not exist
if [ ! -d "./bin" ]; then
    mkdir ./bin
fi

# Copy the release binary to the ./bin folder and rename it to "commit-msg"
cp ./target/release/commitalyzer ./bin/commit-msg
