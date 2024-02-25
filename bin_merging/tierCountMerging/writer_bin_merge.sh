#!/bin/bash
#Change the argument after count to set the tiers

(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "interval" "2" "0C" )


: '
X = time interval
C = catch all
Must specify catch all policy at the end
If dont want the catch all to merge, put "0C" at the end, so no catch all merge policy
Examples....
1) interval 2 0C - aggregate raw data points every 2 seconds and push to the 1st tier. The 1st tier does not merge
2) interval 2 10C - aggregate raw data points every 2 seconds, every 10 seconds merge all the bins in the 1st tier
3) interval 2 10 60C - aggregate raw data points every 2 seconds, every 10 seconds merge all the bins in the 1st tier and push them to the 2nd tier. Every 60 seconds in the 2nd tier merge all the bins collected so far

4) interval 2 10 60 0C - R.D: merge every 2 seconds and puth to t1
                         t1: merge every 10 seconds, push to t2
                         t2: merge every 60 seconds, push to t3
                         t3: No merge policy, so just collect points
'

#"6" "4" "5" "4" "4"