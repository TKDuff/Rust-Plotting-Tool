#!/bin/bash
#Change the argument after count to set the tiers

(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "interval" "7" "5" "10" "120" )


#"6" "4" "5" "4" "4"