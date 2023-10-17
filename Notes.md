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





## Rust GUI's- big concern & hurdle 15-10-2023
Want something that updates live, not hard to learn, exist
* Druid
* Relm
* Iced
* Conrad
* Egui

### Retained Mode
* Persistent tree of objects
* Event bounded

### Immediate Mode - more suitable for this project
* **Stateless**- don't ratain state between frames, entire GUI build between frames
* **Transcient** - GUI rendered online


