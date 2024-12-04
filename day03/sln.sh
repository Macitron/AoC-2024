#!/usr/bin/env bash

infile="$1"
if [[ -z $infile ]]; then
    echo "must specify input file"
    exit 1
fi

echo -n "Mul sum: "
grep -oE 'mul\([0-9]{1,3},[0-9]{1,3}\)' "$infile"           \
    | sed -E 's/.*\(([0-9]{1,3}),([0-9]{1,3})\)/\1 \2/g'    \
    | awk '{s+=($1 * $2)} END {print s}'
