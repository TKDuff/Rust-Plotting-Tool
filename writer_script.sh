#!/bin/bash

ARG=$1

(cd writer && cargo run --bin writer) | (cd strategy_pattern && cargo run --bin strategy_pattern -- "$ARG" )
