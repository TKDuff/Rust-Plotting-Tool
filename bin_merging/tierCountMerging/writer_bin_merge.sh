#!/bin/bash
#Change the argument after count to set the tiers

(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "interval" "7" "10" "60" "8" )


#"6" "4" "5" "4" "4"