#!/usr/bin/bash

# To adapt this script to the new year you are working on
# sed -i -e 's/aoc_{old_year}/aoc_{new_year}/g' -e 's/{old_yearl}/{new_year}/g' ./genday.sh

set -euo pipefail

shopt -s extglob


SCRIPT=$(readlink --canonicalize-existing "$0")
SCRIPT_PATH=$(dirname ${SCRIPT})
[[ -s "${SCRIPT_PATH}/.env" ]] && . "${SCRIPT_PATH}/.env"

generate_day() {
    local day=${1##+(0)}

    if [[ -f "aoc_2021/src/day${day}.rs" ]]; then
        echo "File day${day}.rs already exists"
        exit 1
    fi

    echo "Downloading day ${day} input file"
    curl -# -s "https://adventofcode.com/2021/day/${day}/input"  --cookie "session=${AOC_SESSION}" -o "aoc_2021/data/${day}.txt"

    # printf '$-2i\n        '"${day}"' => aoc_2021::day'"${day}"'::run(),\n.\nw\n' | ed -s aoc_2021/src/main.rs
    echo "Adding match line for day ${day} in main.rs"
    sed -i "/_ =>/i\        ${day} => aoc_2021::day${day}::run()," aoc_2021/src/main.rs

    echo "Appending module for day ${day} in lib.rs"
    echo "pub mod day${day};" >> "aoc_2021/src/lib.rs"

    touch "aoc_2021/src/day${day}.rs"
    echo "Created File day${day}.rs"
}

remove_day() {
    echo "Removing match line from main.rs..."
    printf '$-3d\nw\n' | ed -s aoc_2021/src/main.rs

    echo "Removing module from lib.rs..."
    sed -i '$d' aoc_2021/src/lib.rs
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

while getopts "hr" option; do
    case $option in
        h | --help) 
            help
            exit 0;;
        r | --remove)
            remove_day
            exit 0;;
        \?)
            echo "Error: Invalid option"
            break;;
    esac
done

[[ -z "$1" ]] && { echo "Pass in a day";exit 1; }

generate_day $1

