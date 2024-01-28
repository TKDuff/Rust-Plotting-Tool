# Plotting Tool
P1 | OnlineGraph | P2
- Take data from data stream
- Plots data in real time (renders x,y on graph)
- Run in terminal 
- Run via popen
- Aggregate Rendered Plot
- Use threading to de-couple taking in data, processing and rendering
- Not batch plotting simialr to GNUPlot, online


## Buffer
Pipe capacity fixed at kernel level
As of linux 2.6 pipe capacity is 16 pages (65,365 bytes)
Page - fixed length contigous block of virtual memory

## Thread - so far
Exist three threads, **reader**, **processor** and **render**
### Reader Thread
- Read reads byte stream from the terminal, data from standard input
S- ends data to 'processor' thread
- Reads standard input line by line from terminal (as it comes)
- Data as of now in form of two numbers x,y
- **As of now** no way to read byte stream quickly, just using rust library
- Data (x,y string) passed to processor thread using Multiple Producer Single Consumer library 
- Is the transmiter transmitter in MPSC (tx)
### Processor Thread
- Takes data from Reader thread via MPSC, is receiver (rx)
- Receives single string line, from tx, containing (x,y)  
- Have to get x,y out of string, split string and convert to int
- Print x,y, x average and y average
### Render Thread
- Takes x,y value from Processor thread



## Thread - Ideas
### Reader Thread
- Look into way to increase performance of reading from standard input, pipe may be bottleneck
- Transmit to shared memory? Useful for multiple processor threads working parralel? 

### Processor Thread
- Multiple processor threads? Processing can be done in parralel for each value passed in, process multiple at a time, less queuing to be processed

### Render Thread
- Takes x,y value from Processor thread
- Plots x,y value to GUI
- Can to use Plotter (rust library) inside a GUI, as of now plotter creates .png static image, no way to live plot (yet)

## Aggregate Data
**Use plotter graphic as store of memory**<br>Instead of storing the actual value, reference the plot as a point to get value
<br>
Data not stored in memory, GUI on screen is all the data
<br>
If you want a value from memory, click on Plot (or code reference plot)





## Rust GUI's- big concern & hurdle 15-10-2023, should I use rust?
Want something that updates live, not hard to learn, exist
* Druid - retained
* Relm
* Iced
* Conrad - immediate
* Egui - immediate, native, little documentation
* GTK.rs - retained, non native

### Retained Mode
* Persistent tree of objects
* Event bounded

### Immediate Mode - more suitable for this project
[More information](https://oandre.gal/concepts/immediate-mode-vs-retained-mode/)
* **Stateless**- don't ratain state between frames, entire GUI build between frames
* **Transcient** - GUI rendered online


#### Rust concerns
* I am new to it
* Little documentation for native GUIs (egui, druid) which are most performent
* Documentation exists for non-native GUI, gtk, matplot but not performent
#### Rust advantage
* Memory handling
* Good language to know in the coming years
* Performent


Simple Changes just here to test

## Mutex
external crates that provide advanced and specialized concurrency primitives. Some popular ones include crossbeam, tokio, and async-std.
<br>
**RwLock** - allows multiple threads to read the data simultaneously. If you have a scenario where reads are more frequent than writes





cargo run --bin writer | (cd /home/thomas/FinalYearProject/online-graph/plot_test && cargo run --bin plot_test)

Criterion is rust create to benchmark, measure time


## 4/11/23
Idea now is to have a sliding window, split into three sections as follows
1) Raw Data - raw data from stream plotted, no downsampling, plotted live
2) Transition - Section where raw data is actually downsampled
3) DownSampled - Collection of transition sections forming a continous plot of downsampled data

This form a plot of contigous data, but there exist a cut off between the raw and downsampled. The sliding window determines when raw data is cut-off and downsampled. <br><br>
**You are essentially** moving the downsampling process on the data from before it is plotted (when it directly comes from stdin) to after it is plotted

Think off as split into three windows, the raw data window width is dynamic, once it fills up all the data is moved into the transition window to be downsampled, once downsampled it is moved into the D.S window. 

**Exist Few Problems**
* Transition create bottleneck, if takes a while to downsample the raw data in transition end up blocking R.D window
* Don't know whether to downsample the DownSampled winodow, if
  - **not done** means program memory grows and older data not downsampled, use retained GUI
  - **is done** downsampled then can create bottleneck between Transition, waiting for this section to downsample, **overcome** by having fixed size on this section, thus doesn't grow and fixed downsample time

## 7-11-23 - Version 1<br>
Exist three threads<br>
1) Raw Data, rd - contain 'values' vector, which are values to be plotted
2) Downsample, ds - handles downsampling chunk of the 'values' vector
3) Egui, eg - actual egui window, plotting the values live

* On launch, rawdata module is assigned read write lock
* rd ds and egui threads spawned. 

* rd thread reads in from standard input, and using the write lock appends the read in x,y points to the raw data 'values' vector

* Egui thread constantly reads rawdata 'values' vector, when new value pushed, egui redraws entire plot thus drawing the new points

* ds thread counts read the 'values' vector length, when 10 values pushed
* - ds take a copy of the previous 10 values (from the end) 
  - downsamples them (remove half)
  - Sends the downsampled vector to rd thread via mpsc channel
  - rd (having the write lock) amends the downsampled chunk to the vector

* Both egui and ds both share the read lock, only rd has the write lock
* **Contention does arise when rd has to insert downsampled chunk into 'values' vector, stops reading in from standard input**

**Problem, big problem 9-11-23**
potential efficiency issue with using RwLock in a situation where writes are frequent and reads are constant. This setup can lead to what's known as "write starvation," where the reader(s) may frequently block the writer(s) from accessing the data, especially if there are many reader threads or if the reader holds the lock for a long duration. Additionally, if the downsampler is constantly checking the length of the vector in a tight loop, it can lead to a high contention situation, which is inefficient.

## 12 - 11 - 2023
### Looked into lock-free data structures
* Such as 'crossbeam' which provide such as 'SegQueue', which allow concurrent reads and writes without using locks since they use atomics. 
* All of the lock-free data structures don't support **non-destructive reads or iteration over elements** which means cannot get all elements of vector, simply can't return entire vector, designed for concurrent enqueueing and dequeueing operations 
* Since egui/druid require access to the entire vector to be plotted, as they are data driven, and l.f.d.s don't allow the reading of an entire vector, they cannot be used
* Hence, a RW-lock is the best mechanism for thread-safe concurrency
### Looked into using thread-pools for downsampling
* Instead of single downsampling thread, can use thread pools which will downsamples different chunks coming in from r.d in **parallel**, more performent
* Looked into *Rayon* which is an API that looks after thread pooling
* Looked into standard library thread pooling, will have look into more tomorrow

Key takeway is to stick with RW-lock, can't use lock free data structure that allows good concurrency access to shared datastructure

## 13 - 11- 2023
Going to use a thread pool instead of single D.S thread to downsample. Done if downsample rate slower than rate at which chunks comes in, mutliple threads of pool can downsample in parralel. 
* **Thread pools used primrarily for CPU-bound tasks**, downsampling is CPU instensive, doing math on vector of points. This is why I am not using anync methods for downsampling, async offers concurrency but not parallelism

12-11-23
Get simple crossbeam demo working
Contain r.d thread writing to vector, egui thread reading from vector, should be no locking

Could use DashMap, is a hashmap, could use to store times as keys

So is there no concurrent queue crate in rust that allows non Destructive Reads?I no longer mind if it uses locks or mutual exclusion, anything quicker than a RW lock and not a buffer

* Look into
* * Atomic signal
* * Lock granularity
* * Thread pools 


## 13 - 11- 2023
**To get working**
1) Fix off by one using crossbeam channel send
   * Be sure to drop the locks, using a Reader Get Chunk method, implement crossbeam, not using atomics as no polling
2) Downsample get Descriptive statistics points per second
3) Plot as boxplot, switch to time series

Must choose between the following statistical libraries in rust
statrs - https://github.com/statrs-dev/statrs
incr_stats - https://github.com/garyboone/incr_stats
Using plotters for now, no crate to outright get quartiles

Look into using this **ndarray** - https://docs.rs/ndarray/latest/ndarray/

## 5-12-2023 
**Problem with current buffer method** 
* R (the R.D.T)
* H sees length is 10
* Set flag true
* R check condition, see R true
* Crucially, while this check occurs, since H is in a loop, it sees the length is still 10, so it beings the downsample process during the remove
* So R removes the points (keep in mind H is still downsampling)
* R sets the flag to false
* Crucially, when R finishes the remove and sets the flag to false, H sets the downsample flag to true (since it fished the downsample) right after
* Now the flag is true again
* R appends a point
* R sees the flag is true (based on when H saw the length was 10)
* R tries to remove but failes

**H always see stale data, here is where problem lies**
* R.D has the live view of the data
* Since H.D reads the data from R.D vector, it will always have a stale view of the data
* Currently, H.D polls length of R.D vector, downsamples, then tells R.D to delete chunk
* Since has a view of stale data, therefore always risk of race condition

## Solution - change R.D concerns
* R.D now writes and checks length, R.D thread has full access to R.D thread## 17 - 12 - 2023

Race Condition Window: There's still a small window for a race condition. After H checks the flag and before it starts the downsampling, R could potentially change the state. This window might be small, but in highly concurrent systems, even tiny windows can lead to issues.

Dependency on Flag State: This approach heavily relies on the accurate and timely update of the PROCESS_FLAG. Any delay or missed update could lead to incorrect behavior.

Stale Data Risk: If R is appending data very quickly, there's a risk that H might act on stale data. When H decides not to downsample because the flag is true, R might have already appended more data, changing the situation by the time H checks again.

## 17 - 12 - 2023
data streams in from standard input, the data is in the form of two floating points numbers think of as x,y. The async thread (call it r.d) appends the x,y values to a vector, called 'points'. The async thread monitors the length, after N amount of points (lets say 10 points) are added, another thread, (call it h.d) aggregates the chunk of points and appends the aggregate statistics to another vector, called 'statistics'. Once the aggregate stats for a chunk is appended to 'statistics' vector, the async thread r.d. removes the chunk of points.
Think of r.d as a sliding window, when certain amount read in removes the last chunk while reading in new points at the same time.

The egui thread plots both the 'points' vector with the raw data steaming in and being deleted and the statistics vector, with the aggregate statistics being added incrementally (thus removing the the chunk it represents from 'points')

## 24 - 01 - 2023
Considered using **Estimated Moving Average** which 
* tracks the average value of a series over time
* Calculates average for all points, giving more weight to more recent data, less to older. So is over entire data set not just window
* Plots trends more similar raw data as it comes in, also not waiting on window to fill up, get average as point comes in
* Average is for entire collection of points, not a bucket, so more representative of entire trend

How ever decided to not use it now, continue using buckets for now, may implement later as...
1) Only calculate average, not statistical values which are important and can be done via sliding window
2) Does not work aggregate all points, just gets average
3) While smoother, not too far off average for a winow

##### Will use EMA near end, as a user option, as a **hybrid** appraoch
So the EMA is calculated in parralel, when a bucket is full all the stats will be obtained for that bucket (min,max,count) but the average won't be calculate from the points in the bucket alone, it will be a snapshot of the EMA at the time the bucket is filled/created

EMA would provide a smooth trend line that is continuously updated with each new data point

## 25-01-23
Spent the day looking at ways to dynamically adjust the tumbling window size. The window size is static, this is not good in cases when the variance fluctuates. A shorter window is necessary for high variance to extract detail, while a larger window is need for low variance since not much to represent.
Will add times aggregation, but this seems most important. 
Planning to use r.d plot to capture variance, then with live variance change the tumbling window size. 

### LOOK INTO PAGE HINKLEY METHOD

## 26-01-23
Will need to write, look into Adaptive Window

## 27-01-23
Will need to write, look into Adaptive Window

## 28-01-23 
Getting ADWIN to work with Rust
Got github Adwin (https://github.com/Patrick-Harned/adwin/tree/master) to work,however not useful as
* Uses a sliding, not tumbling, window
* Window size remains constant
* Mainly used to detect drift which is used for ML, not for aggregating the window

Thus will have to create my own

Got a working version on ADWIN in rust, does not plot to EGUI but aggregates.
Have window, split into 2, N1 and N2. N1 is aggregated and plotted while N2 becomes the new tumbling window