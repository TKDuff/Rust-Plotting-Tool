#!/bin/bash
#Change the argument after count to set the tiers

#(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging "count" "40" "10" "100C2")
#(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "count" "5" "5" "5" "0C0"  )
#(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "count" "20" "60" "6" "10C2"  )
#(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "interval" "1S" "0C0"  )
(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "interval" "1S" "10SC100"  )
#(cd ../../writer/ && cargo run --bin writer) | (cargo run --bin tierCountMerging -- "interval" "1S" "10S" "1M" "15MC2"  )




: '
Count based merging
Consist of following vectors
1) initial tier of stdin data streaming in
2) Merging tiers
3) Final catch all tier

Each tier specify number of elements it must contain to merge vector.
Final tier, catch all tier, specify number of elements to chunk the vector and the chunk size. 
If dont want to gradually summarise final tier, can put 0C0 at the end, to specify no catch all policy. Will grow over time
Examples....

1) count 5 0C0 - every 5 stanrd input points aggregate them into a bin and push to the catch all tier. Every 10 points in the final tier, merge the bins in chunks of three

2) count 5 10 0C0 - every 5 standard input points aggregate them into a bin and push to next tier
                    every 10 bins obtained merge them into a sinlge bin and push to next, final tier
                    no policy to merge bins, collect more over time

3) count 2 10 20C3 - every 2 standard input points aggregate them into a bin and push to next tier
                     every 10 bins obtained merge them into a sinlge bin and push to next, final tier
                     every 20 bis obtained, merge every 3 bins. Result in 6 merged and 1 remainder bin


Interval merging
Must specify catch all policy at the end
If dont want the catch all to merge, put "0C0" at the end, so no catch all merge policy

S - second
M - minute
H - hour

Examples....
1) interval 2S 0C0 - aggregate raw data points every 2 seconds and push to the 1st tier. The 1st tier does not merge
2) interval 2S 10SC - aggregate raw data points every 2 seconds, every 10 seconds merge all the bins in the 1st tier
3) interval 2S 10S 1MC - aggregate raw data points every 2 seconds, every 10 seconds merge all the bins in the 1st tier and push them to the 2nd tier. Every 60 seconds (1 minute) in the 2nd tier merge all the bins collected so far

4) interval 2S 10S 1M 0C0 - R.D: merge every 2 seconds and puth to t1
                         t1: merge every 10 seconds, push to t2
                         t2: merge every 60 seconds, push to t3
                         t3: No merge policy, so just collect points



Same applies to count, however it is not based on the number of seconds, instead the number of elements in the Raw Data vector and Bins in the tier vector
'

#"6" "4" "5" "4" "4"

