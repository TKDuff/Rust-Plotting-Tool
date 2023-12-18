import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Load your dataset from the CSV file
df = pd.read_csv('Generated_Reward_Data_10000.csv')

# Process the first 2000 points
df_subset = df.iloc[:2000]

# Calculate the average x and y values for every 20 instances
bin_size = 20
averaged_data = {
    'Average Episode No.': df_subset['Episode No.'].groupby(df_subset.index // bin_size).mean(),
    'Average Reward': df_subset['Reward'].groupby(df_subset.index // bin_size).mean()
}

# Create a new DataFrame from the averaged data
averaged_df = pd.DataFrame(averaged_data)

# Convert the Series to NumPy arrays for plotting
avg_episode_numbers = averaged_df['Average Episode No.'].to_numpy()
avg_rewards = averaged_df['Average Reward'].to_numpy()

# Plotting the new dataframe of 100 points
plt.figure(figsize=(10, 5))
plt.plot(range(1, 101), avg_rewards)
plt.title('Average Reward per 20 Episodes (First 2000 Episodes)')
plt.xlabel('Average Episode No.')
plt.ylabel('Average Reward')
plt.grid(True)
plt.show()
