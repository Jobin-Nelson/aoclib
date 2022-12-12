#!/usr/bin/bash

# To adapt this script to the new year you are working on
# sed -i -e 's/aoc_{old_year}/aoc_{new_year}/g;s/{old_yearl}/{new_year}/g' genday.sh

set -euo pipefail

shopt -s extglob

SCRIPT=$(readlink --canonicalize-existing "$0")
SCRIPT_PATH=$(dirname "${SCRIPT}")

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

    if (( day <= 0 || day > 25 )); then
        echo "Invalid date provided = $day"
        echo "Date must be between 1 and 25 inclusive"
        exit 1
    fi

    printf -v zfill_day "%02d" "$day"

    local day_path="${SCRIPT_PATH}/aoc_2022/data/day${zfill_day}.txt"

    if [[ -f $day_path ]]; then
        echo "File day${zfill_day}.txt already exists"
        exit 1
    fi

    echo "Downloading day ${day} input file..."
    mkdir -p "${SCRIPT_PATH}/aoc_2022/data"
    curl -# -s "https://adventofcode.com/2022/day/${day}/input"  --cookie "session=${AOC_SESSION}" -o "$day_path"
    echo "Input file stored in $day_path"
}

function remove() {
    local day=${1##+0}

    if (( day <= 0 || day > 25 )); then
        echo "Invalid date provided = $day"
        echo "Date must be between 1 and 25 inclusive"
        exit 1
    fi

    printf -v zfill_day "%02d" "$day"

    local day_path="${SCRIPT_PATH}/aoc_2022/data/day${zfill_day}.txt" 

    if [[ -f $day_path ]];then
        rm "$day_path"
    fi

    echo "File removed for day = $day"
}

function help() {
    echo
    echo "Setups up everything  for the given day"
    echo
    echo "Usage: genday.sh day [-h] [-d]"
    echo "Options:"
    echo "day   Download and setup files for that day"
    echo "h     Print this help"
    echo "d     Delete the input file for that day"
    echo
}

while getopts "hd:" option; do
    case $option in
        h) 
            help
            exit 0;;
        d)
            remove "$OPTARG"
            exit 0;;
        \?)
            echo "Error: Invalid option"
            break;;
    esac
done

[[ -z "$1" ]] && { echo "Pass in a day";exit 1; }

generate_day "$1"

