#!/bin/bash

# Create input files for all 25 days
mkdir -p inputs

for day in {1..25}; do
    input_file="inputs/${day}.txt"
    if [[ ! -f $input_file ]]; then
        echo "Creating template input file: $input_file"
        touch "$input_file"
    else
        echo "File already exists: $input_file"
    fi
done
