#!/usr/bin/env bash

cargo run --release > output.txt

./prepare-csv.py

