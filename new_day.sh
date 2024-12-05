#!/bin/bash
day=$(date +%d)
day=$(echo $day | sed 's/^0//')
crate_name="d$(printf "%02d" $day)"

cargo new $crate_name
cp template/main.rs $crate_name/src/main.rs
