#!/bin/bash
day=$(date +%d)
crate_name="d$day"

cargo new $crate_name
cp template/main.rs $crate_name/src/main.rs
cp template/README.md $crate_name/README.md

sed -i "s/{CRATE_NAME}/$crate_name/g" $crate_name/README.md
sed -i "s/{DAY_NO}/$(echo $day | sed 's/^0*//')/g" $crate_name/README.md
