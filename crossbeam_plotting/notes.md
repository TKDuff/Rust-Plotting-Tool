Using crossbeam to send data from R.D thread to D.S thread


**Not doing anymore**
<br>
r.d signal to d.s via channel to downsample, not using atomic flags as d.s will be busy waiting polling the flag frequently
Message contain both signal and chunk to downsample, thus more compact, d.s doesn't need to read r.d vector constantly. 

**Doing**<br>
Use lock free data structure, D.S read R.D vector continously, when 50 points added D.S take a copy, downsample it.

Going to use a thread pool instead of single D.S thread to downsample. Done if downsample rate slower than rate at which chunks comes in, mutliple threads of pool can downsample in parralel. 
* **Thread pools used primrarily for CPU-bound tasks**, downsampling is CPU instensive, doing math on vector of points. This is why I am not using anync methods for downsampling, async offers concurrency but not parallelism

12-11-23
Get simple crossbeam demo working
Contain r.d thread writing to vector, egui thread reading from vector, should be no locking

Could use DashMap, is a hashmap, could use to store times as keys

So is there no concurrent queue crate in rust that allows non Destructive Reads?I no longer mind if it uses locks or mutual exclusion, anything quicker than a RW lock and not a buffer