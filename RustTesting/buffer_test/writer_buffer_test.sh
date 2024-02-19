#!/bin/bash


(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin buffer_test )
