import pandas as pd
import numpy as np

# Set a random seed for reproducibility
np.random.seed(0)

# Generate 400 sequential numbers for the x-axis (Episode No.)
episode_numbers = np.arange(1, 401)

# Generate granular data with a general upward trend for the y-axis (Reward)
# We'll use a combination of a linear trend and some noise.
trend = np.linspace(0, 20, 400)  # Linear trend upwards
noise = np.random.normal(0, 2, 400)  # Random noise
rewards = trend + noise

# Create a DataFrame
df = pd.DataFrame({
    'Episode No.': episode_numbers,
    'Reward': rewards
})

# Save to a CSV file
df.to_csv('Generated_Reward_Data.csv', index=False)
