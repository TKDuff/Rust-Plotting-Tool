import pandas as pd
import matplotlib.pyplot as plt
import statistics

# Load the CSV file
file = 'variance_dataset.csv'
df = pd.read_csv("/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file))

# Initialize arrays to store the plot data
x_data = []
y_data = []

# Initialize variables for calculating streaming variance
n = 0  # Number of data points read
mean = 0.0  # Current mean
M2 = 0.0  # Current M2 (used to calculate variance)

# Set up the plot
plt.ion()  # Enable interactive mode
fig, ax = plt.subplots()
line, = ax.plot([], [], 'b-')  # Initialize an empty line
ax.set_xlabel('x_col')
ax.set_ylabel('y_col')
ax.set_title('Streaming Data with Variance Calculation')

# Function to update the plot
def update_plot(x, y):
    x_data.append(x)
    y_data.append(y)
    line.set_data(x_data, y_data)
    ax.relim()
    ax.autoscale_view()
    fig.canvas.draw()
    fig.canvas.flush_events()

# Read and plot the data points
for i in range(len(df)):
    x_value = df['x_col'].iloc[i]
    y_value = df['y_col'].iloc[i]

    # Update streaming variance
    n += 1
    delta = y_value - mean
    mean += delta / n
    delta2 = y_value - mean
    M2 += delta * delta2

    # Add the new point to the data arrays and update the plot
    update_plot(x_value, y_value)

    # Print streaming variance
    if n > 1:
        variance = M2 / (n - 1)
        print(f"Point {n}: Streaming Variance = {variance:.2f}")

# Disable interactive mode and display the final plot
plt.ioff()
plt.show()
