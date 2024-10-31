#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <filename>"
    exit 1
fi

echo "Compiling $1.rs"
rustc -o target/$1 src/$1.rs
echo "$1.rs compiled!"