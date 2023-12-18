import matplotlib.pyplot as plt
from matplotlib.widgets import Button
import numpy as np
import lttb

# Function to handle downsampling button click
def downsample_data(event):
    downsampled_data = lttb.downsample(np.array([x_values, y_values]).T, n_out=10)
    line.set_xdata(downsampled_data[:, 0])
    line.set_ydata(downsampled_data[:, 1])
    plt.draw()

# Function to handle raw data button click
def show_raw_data(event):
    line.set_xdata(x_values)
    line.set_ydata(y_values)
    plt.draw()

# Generate example data
x_values = np.linspace(0, 100, 100)
y_values = np.sin(x_values) + np.random.normal(0, 0.1, 100)

# Create plot
fig, ax = plt.subplots()
plt.subplots_adjust(bottom=0.25)
line, = ax.plot(x_values, y_values, lw=2)

# Add buttons for downsampling and showing raw data
ax_button_downsample = plt.axes([0.5, 0.05, 0.2, 0.075])
btn_downsample = Button(ax_button_downsample, 'Downsample')
btn_downsample.on_clicked(downsample_data)

ax_button_raw = plt.axes([0.7, 0.05, 0.2, 0.075])
btn_raw = Button(ax_button_raw, 'Raw Data')
btn_raw.on_clicked(show_raw_data)

plt.show()

