#!/bin/bash


(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "count" "3" )
