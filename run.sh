#!/usr/bin/env bash

set -euo pipefail

cargo build --release

last_day=25

function run() {
    for i in `seq 1 $last_day`; do
        echo "Day $i"
        target/release/day$i input/input$i.txt
        echo
    done
}

time run
