import pandas as pd
import numpy as np

# Set a random seed for reproducibility
np.random.seed(0)

# Generate 10,000 sequential numbers for the x-axis (Episode No.)
episode_numbers = np.arange(1, 10001)

# Generate granular data with a general upward trend for the y-axis (Reward)
# We'll use a combination of a linear trend and some noise.
trend = np.linspace(0, 50, 10000)  # Linear trend upwards
noise = np.random.normal(0, 2, 10000)  # Random noise

# Create a smooth dip around 1000 and 4000 points
dip = -15 * np.exp(-0.001 * (episode_numbers - 2500)**2) - 15 * np.exp(-0.0005 * (episode_numbers - 7000)**2)

# Combine trend, noise, and dip
rewards = trend + noise + dip

# Create a DataFrame
df = pd.DataFrame({
    'Episode No.': episode_numbers,
    'Reward': rewards
})

# Save to a CSV file
df.to_csv('Generated_Reward_Data_10000.csv', index=False)
