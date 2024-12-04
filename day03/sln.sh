#!/usr/bin/env bash

infile="$1"
if [[ -z $infile ]]; then
    echo "must specify input file"
    exit 1
fi

sum=0
while read -r mul; do
    (( sum += $(echo "$mul" | sed -E 's/.*\(([0-9]{1,3}),([0-9]{1,3})\)/\1 * \2/g') ))
done < <(grep -oE 'mul\([0-9]{1,3},[0-9]{1,3}\)' "$infile")

echo "Mul sum: $sum"
