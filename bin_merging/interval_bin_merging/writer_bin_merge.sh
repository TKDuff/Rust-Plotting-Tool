#!/bin/bash


(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin interval_bin_merging -- "count" )
