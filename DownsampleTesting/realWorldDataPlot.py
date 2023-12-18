import pandas as pd
import matplotlib.pyplot as plt

# Load the dataset
df = pd.read_csv('Generated_Reward_Data_10000.csv')

# Slice the DataFrame to keep only the first 500 rows
df_subset = df.iloc[:9995]

# Convert the Series to NumPy arrays
episode_numbers = df_subset["Episode No."].to_numpy()
rewards = df_subset["Reward"].to_numpy()

# Plotting the data without markers
plt.figure(figsize=(10, 5))
plt.plot(episode_numbers, rewards)
plt.title('Rewards per Episode (First 500 Points)')
plt.xlabel('Episode No.')
plt.ylabel('Reward')
plt.grid(True)
plt.show()