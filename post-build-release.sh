#!/bin/bash

# Create ./bin directory if it does not exist
if [ ! -d "./bin" ]; then
    mkdir ./bin
fi

# Check if file was updated.
contin=true
if [ -f "./bin/commit-msg" ]; then
    old="$(cat ./target/release/commitalyzer)"
    new="$(cat ./bin/commit-msg)"
    if [ "$old" == "$new" ]; then
        contin=false
    fi
fi

# If file was updated, copy the release binary to the ./bin folder and rename it to "commit-msg"
if [ $contin == true ]; then
    echo "Updating the release binary in root since binary has been modified since last release"
    cp ./target/release/commitalyzer ./bin/commit-msg
fi
