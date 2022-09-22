#!/usr/bin/bash

# To adapt this script to the new year you are working on
# sed -i -e 's/aoc_{old_year}/aoc_{new_year}/g' -e 's/{old_yearl}/{new_year}/g' ./genday.sh

set -eo pipefail

shopt -s extglob

[[ -s .env ]] && . .env

generate_day() {
    local day=${1##+(0)}

    if [[ -f "aoc_2021/src/day${day}.rs" ]]; then
        echo "File day${day}.rs already exists"
        exit 1
    fi

    curl -s "https://adventofcode.com/2021/day/${day}/input"  --cookie "session=${AOC_SESSION}" -o "aoc_2021/data/${day}.txt"

    # printf '$-2i\n        '"${day}"' => aoc_2021::day'"${day}"'::run(),\n.\nw\n' | ed -s aoc_2021/src/main.rs
    sed -i "/_ =>/i\        ${day} => aoc_2021::day${day}::run()," aoc_2021/src/main.rs
    echo "pub mod day${day};" >> "aoc_2021/src/lib.rs"
    touch "aoc_2021/src/day${day}.rs"

    echo "Created File day${day}.rs"
}

help() {
    echo
    echo "Setups up everything  for the given day"
    echo
    echo "Usage: genday.sh day [-h]"
    echo "Options:"
    echo "h     Print this help"
    echo "day   Download and setup files for that day"
    echo
}

while getopts "h" option; do
    case $option in
        h | --help) 
            help
            exit 0;;
        \?)
            echo "Error: Invalid option"
            break;;
    esac
done

[[ -z "$1" ]] && { echo "Pass in a day";exit 1; }

generate_day $1

