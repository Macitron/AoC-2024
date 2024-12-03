#!/usr/bin/env bash

# nth column: cat input | awk '{print $n}'
# sorted columns side-by-side: paste <(cat input | awk '{print $1}' | sort) <(cat input | awk '{print $2}' | sort)

infile=$1
if [[ -z $infile ]]; then
    echo 'must specify input file!'
    exit 1
fi

echo -n "Distance: "
paste <(awk '{ print $1 }' "$infile" | sort) <(awk '{ print $2 }' "$infile" | sort) \
    | awk '{print ($2 - $1)}'   \
    | sed -E 's/^-//g'          \
    | awk '{s+=$1} END {print s}'

echo -n "Similarity Score: "
join <(awk '{print $1}' "$infile" | sort) <(awk '{print $2}' "$infile" | sort) \
    | uniq -c   \
    | awk '{s+=($1 * $2)} END {print s}'
