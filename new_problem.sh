#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "Usage: `basename $0` <PROBLEM_NUMBER>"
    exit 1
fi

filename_src=$(printf "./src/bin/problem%03d.rs" $1)

cat << RUST > $filename_src
fn main() {
    println!("Hello, world!");
}
RUST
