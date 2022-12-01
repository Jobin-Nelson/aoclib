#!/usr/bin/bash

# To adapt this script to the new year you are working on
# sed -i -e 's/aoc_{old_year}/aoc_{new_year}/g;s/{old_yearl}/{new_year}/g' genday.sh

set -euo pipefail

shopt -s extglob

SCRIPT=$(readlink --canonicalize-existing "$0")
SCRIPT_PATH=$(dirname "${SCRIPT}")
echo "$SCRIPT_PATH"

[[ -s "${SCRIPT_PATH}/.env" ]] && . "${SCRIPT_PATH}/.env"

if ! [[ -d "${SCRIPT_PATH}/aoc_2022" ]]; then
    echo "No aoc_2022 crate found"
    exit 1
fi

if [[ -z $AOC_SESSION ]]; then
    echo "No AOC_SESSION api key found"
    exit 1
fi

function generate_day() {
    local day=${1##+0}
    printf -v zfill_day "%02d" "$day"

    if [[ -f "${SCRIPT_PATH}/aoc_2022/src/bin/day${zfill_day}.rs" ]]; then
        echo "File day${zfill_day}.rs already exists"
        exit 1
    fi

    echo "Downloading day ${day} input file..."
    mkdir -p "${SCRIPT_PATH}/aoc_2022/data"
    curl -# -s "https://adventofcode.com/2022/day/${day}/input"  --cookie "session=${AOC_SESSION}" -o "${SCRIPT_PATH}/aoc_2022/data/day${zfill_day}.txt"
    echo "Input file stored in data/day${zfill_day}.txt"

    touch "${SCRIPT_PATH}/aoc_2022/src/bin/day${zfill_day}.rs"
    echo "Created File day${zfill_day}.rs"
}

function help() {
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
        h) 
            help
            exit 0;;
        \?)
            echo "Error: Invalid option"
            break;;
    esac
done

[[ -z "$1" ]] && { echo "Pass in a day";exit 1; }

generate_day "$1"

