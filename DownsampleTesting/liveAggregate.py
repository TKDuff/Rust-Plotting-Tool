import pandas as pd
import matplotlib.pyplot as plt
import time

# Load the CSV file
df = pd.read_csv('Generated_Reward_Data_10000.csv')

# Initialize arrays to store the plot data
x_data = []
y_data = []

# Set up the plot
plt.ion()  # Enable interactive mode
fig, ax = plt.subplots()
line, = ax.plot(x_data, y_data, 'r-')  # Initialize an empty line
ax.set_xlim(0, 2000)  # Set the x-axis limit
ax.set_ylim(min(df['Reward']), max(df['Reward']))  # Set the y-axis limit

# Live plot and replace every 20 elements with their mean
for i in range(2000):
    # Add the new point to the data arrays
    x_data.append(df['Episode No.'].iloc[i])
    y_data.append(df['Reward'].iloc[i])

    # Update the plot
    line.set_data(x_data, y_data)
    fig.canvas.draw()
    fig.canvas.flush_events()

    # Replace every 20 points with their mean
    if (i + 1) % 20 == 0:
        mean_x = sum(x_data[-20:]) / 20
        mean_y = sum(y_data[-20:]) / 20
        # Replace the last 20 points with the mean
        x_data[-20:] = [mean_x] * 20
        y_data[-20:] = [mean_y] * 20
        print(f"Mean X: {mean_x}, Mean Y: {mean_y}")

    time.sleep(0.01)  # Adjust the speed of plotting here

plt.ioff()  # Disable interactive mode
plt.show()
