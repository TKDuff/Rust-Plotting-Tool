#!/bin/bash
#Change the argument after count to set the tiers

(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "count" "6" "6" "6" )
