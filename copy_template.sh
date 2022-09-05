#!/bin/bash

for n in $(seq 1 25) ; do
    if [ "src/days/day$ns.rs" -nt ".gitignore" ] ; then
        # skip files that have been edited since the repo was cloned
        continue
    fi
    ns=$(printf "%02d" $n)
    sed "s/00/$ns/" day_template.rs >src/days/day$ns.rs
done
