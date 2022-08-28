#!/bin/bash

for n in $(seq 1 31) ; do
    if [ n == 2 ] ; then continue; fi
    ns=$(printf "%02d" $n)
    sed "s/00/$ns/" day_template.rs >src/days/day$ns.rs
done
