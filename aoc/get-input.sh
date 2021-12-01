#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo "Provide day number (1-30)"
    exit 1
fi

cookie=$(cat ./cookie.env)
day=$1
curl "https://adventofcode.com/2021/day/$day/input" --cookie "$cookie" --output "inputs/$day.txt"
