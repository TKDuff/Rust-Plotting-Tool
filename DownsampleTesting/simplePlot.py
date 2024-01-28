import pandas as pd
import matplotlib.pyplot as plt

# Load the CSV file
#file = 'Generated_Reward_Data_10000.csv'
#file = 'LunarLander-v2_Reward.csv'
#file = 'slow_increase_with_chunks.csv'
file= 'RustTest.csv'
#file = 'variance_dataset.csv'
df = pd.read_csv("/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % file)

# Extract data from the DataFrame
x_data = df['x_col'].to_numpy()  # Convert to NumPy array
y_data = df['y_col'].to_numpy()  # Convert to NumPy array

# Set up the plot
fig, ax = plt.subplots()
line, = ax.plot(x_data, y_data, 'r-')  # Initialize an empty line
ax.set_xlim(0, 500)  # Set the x-axis limit
ax.set_ylim(min(y_data), max(y_data))  # Set the y-axis limit

# Display the plot
plt.show()
