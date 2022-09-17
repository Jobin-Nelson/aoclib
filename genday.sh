#!/usr/bin/bash

set -euxo pipefail

generate_day() {
    echo "hi"
}

help() {
    echo "Setups up everything  for the given day"
    echo
    echo "Usage: genday.sh [-h]"
    echo "Options:"
    echo "h     Print this help"
    echo
}

while getopts "d:h" option; do
    case $option in
        d)
            generate_day "$OPTARG"
            exit 0;;
        h) 
            help
            exit 0;;
        \?)
            echo "Error: Invalid option"
            exit 1;;
    esac
done
