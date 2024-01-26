import pandas as pd
import numpy as np
import time


# Load the dataset
file = 'variance_dataset.csv'
df = pd.read_csv("/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file))

# Initialize the default window size
window_size = 20  # A starting point; can be adjusted based on further analysis

# Get the initial data points up to the window size
initial_data = df['y_col'].iloc[:window_size]

# Calculate the variance of the initial window
base_variance = np.var(initial_data)
print(f"Initial window size: {window_size}")
print(f"Base variance: {base_variance}")


# Loop through each point in the dataset beyond the initial window
for i in range(window_size, len(df['y_col'])):
    # Update the current window with the new point
    current_window = df['y_col'].iloc[i-window_size+1:i+1]
    
    # Calculate the variance of the current window
    current_variance = np.var(current_window)

    # Compare the current variance with the base variance
    print(f"Current variance {i}: {current_variance} (Base variance: {base_variance})")

    # Sleep for 0.5 seconds to simulate streaming
    time.sleep(0.5)
