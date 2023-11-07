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
