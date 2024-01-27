import pandas as pd
import matplotlib.pyplot as plt

# Load the CSV file
file = 'variance_dataset.csv'
df = pd.read_csv("/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file))

# Convert DataFrame columns to NumPy arrays
x_data = df['x_col'].to_numpy()
y_data = df['y_col'].to_numpy()

# Calculate the average of every 20 points and store in the new arrays
x_avg_data = []
y_avg_data = []
chunk_size = 4

for i in range(0, len(df), chunk_size):
    x_avg = df['x_col'].iloc[i:i+chunk_size].mean()
    y_avg = df['y_col'].iloc[i:i+chunk_size].mean()
    x_avg_data.append(x_avg)
    y_avg_data.append(y_avg)

# Set up the plot for the original data
plt.figure(figsize=(10, 5))  # Create a larger figure
plt.plot(x_data, y_data, 'b-', label='Original Data')  # Plot the original data
plt.plot(x_avg_data, y_avg_data, 'r-', label=f"Average of Every {chunk_size} Points")  # Plot the average data
plt.xlabel('x_col')
plt.ylabel('y_col')
plt.title('Original Data and Average of Every 20 Points')
plt.legend()  # Show legend to distinguish between the two lines

# Display the plot
plt.show()
