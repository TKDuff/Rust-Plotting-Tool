
import matplotlib.pyplot as plt
import numpy as np

# Generate example data
x = np.linspace(0, 10, 100)
y = np.sin(x) + np.random.normal(0, 0.1, 100)

# Define the ratio of detailed to simplified data
simplified_ratio = 0.8  # First 80% is simplified
detailed_ratio = 0.2    # Last 20% is detailed

# Calculate the index where the data will transition from simplified to detailed
transition_index = int(len(x) * simplified_ratio)

# Simplify the first 80% of the data by averaging every five points
num_points_to_average = 5
simplified_x = x[:transition_index:num_points_to_average]
simplified_y = [np.mean(y[i:i + num_points_to_average]) for i in range(0, transition_index, num_points_to_average)]

# The last 20% of the data will be detailed
detailed_x = x[transition_index:]
detailed_y = y[transition_index:]

# Create the plot
plt.figure(figsize=(10, 5))

# Plot the simplified data
plt.plot(simplified_x, simplified_y, 'o-', label='Simplified Data')

# Plot the detailed data
plt.plot(detailed_x, detailed_y, 'o-', label='Detailed Data')

# Add a vertical line to indicate the transition point
plt.axvline(x=x[transition_index], color='red', linestyle='--', label='Detail Transition')

# Adding labels and title
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.title('Hybrid Detail Plot')
plt.legend()

# Show the plot
plt.show()