#!/bin/bash

input_path=$1

if [ ! -f $input_path ]; then
    echo "file not found!"
    exit 1
fi

output_path="target/${input_path#src/}"
output_path=${output_path%.*}

mkdir -p $(dirname $output_path)

rustc $input_path -o $output_path
if [ $? -ne 0 ]; then
    exit 1
fi

./$output_path
