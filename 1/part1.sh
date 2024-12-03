#!/usr/bin/env bash

# nth column: cat input | awk '{print $n}'
# sorted columns side-by-side: paste <(cat input | awk '{print $1}' | sort) <(cat input | awk '{print $2}' | sort)

paste <(cat input | awk '{ print $1 }' | sort) <(cat input | awk '{ print $2 }' | sort) \
    | awk '{print ($2 - $1)}'   \
    | sed -E 's/^-//g'          \
    | awk '{s+=$1} END {print s}'
