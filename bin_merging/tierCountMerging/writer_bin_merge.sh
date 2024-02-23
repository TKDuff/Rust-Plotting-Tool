#!/bin/bash
#Change the argument after count to set the tiers

(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "count" "7" "6" "8" )


#"6" "4" "5" "4" "4"