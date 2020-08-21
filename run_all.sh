#!/bin/bash

for program in ./target/release/problem???; do
    echo -n "${program:24:3} "
    "${program}"
done
