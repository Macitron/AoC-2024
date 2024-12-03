#!/usr/bin/env bash

# nth column: cat input | awk '{print $n}'
# sorted columns side-by-side: paste <(cat input | awk '{print $1}' | sort) <(cat input | awk '{print $2}' | sort)

echo -n "Distance: "
paste <(awk '{ print $1 }' input | sort) <(awk '{ print $2 }' input | sort) \
    | awk '{print ($2 - $1)}'   \
    | sed -E 's/^-//g'          \
    | awk '{s+=$1} END {print s}'

echo -n "Similarity Score: "
join <(awk '{print $1}' input | sort) <(awk '{print $2}' input | sort) \
    | uniq -c   \
    | awk '{s+=($1 * $2)} END {print s}'
