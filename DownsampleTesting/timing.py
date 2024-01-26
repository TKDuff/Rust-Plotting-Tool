import pandas as pd
import matplotlib.pyplot as plt
import numpy as np  # Import numpy for variance calculation

# Load the CSV file
file = 'LunarLander-v2_Reward.csv'
#file = 'Generated_Reward_Data_10000.csv'
df = pd.read_csv("/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % file)

# Initialize arrays to store the plot data
x_data = []
y_data = []

# Set up the plot
plt.ion()  # Enable interactive mode
fig, ax = plt.subplots()
line, = ax.plot(x_data, y_data, 'r-')  # Initialize an empty line
ax.set_xlim(0, 2000)  # Set the x-axis limit
ax.set_ylim(min(df['Reward']), max(df['Reward']))  # Set the y-axis limit

# Function to determine the dynamic window size
def calculate_window_size(data, min_window=10, max_window=50, threshold=1000):  # Adjusted threshold
    variance = np.var(data[-max_window:])
    if variance > threshold:
        # Adjust the window size more dynamically based on variance
        new_window = int(max_window / (variance / threshold))
        return max(min_window, min(new_window, max_window)), variance
    else:
        return max_window, variance


# Live plot and dynamically replace points with their mean
for i in range(2000):
    # Add the new point to the data arrays
    x_data.append(df['Episode No.'].iloc[i])
    y_data.append(df['Reward'].iloc[i])

    # Update the plot
    line.set_data(x_data, y_data)
    fig.canvas.draw()
    fig.canvas.flush_events()

    # Dynamic window sizing
    if len(y_data) >= 10:  # Ensure there's enough data for variance calculation
        window_size, variance = calculate_window_size(y_data)
        if len(y_data) % window_size == 0:
            mean_x = sum(x_data[-window_size:]) / window_size
            mean_y = sum(y_data[-window_size:]) / window_size
            x_data[-window_size:] = [mean_x] * window_size
            y_data[-window_size:] = [mean_y] * window_size

            # Print the variance and window size
            print(f"Aggregation occurred - Variance: {variance}, Window Size: {window_size}")

    # Uncomment to control the speed of plotting
    # time.sleep(0.01)

plt.ioff()  # Disable interactive mode
plt.show()
