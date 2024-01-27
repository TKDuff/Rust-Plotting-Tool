import pandas as pd
import numpy as np

# Create an empty DataFrame
data = pd.DataFrame(columns=['Instance', 'Value'])

# Generate 100 instances with low variance
low_variance_chunk = np.random.normal(50, 5, 100)
data = pd.concat([data, pd.DataFrame({'Instance': range(len(data), len(data) + 100), 'Value': low_variance_chunk})], ignore_index=True)

# # Generate 100 instances with high variance
# high_variance_chunk = np.random.normal(50, 25, 100)
# data = pd.concat([data, pd.DataFrame({'Instance': range(len(data), len(data) + 100), 'Value': high_variance_chunk})], ignore_index=True)

# # Generate 100 more instances with low variance
# low_variance_chunk = np.random.normal(50, 5, 100)
# data = pd.concat([data, pd.DataFrame({'Instance': range(len(data), len(data) + 100), 'Value': low_variance_chunk})], ignore_index=True)

# # Generate 100 more instances with high variance
# high_variance_chunk = np.random.normal(50, 25, 100)
# data = pd.concat([data, pd.DataFrame({'Instance': range(len(data), len(data) + 100), 'Value': high_variance_chunk})], ignore_index=True)

# # Generate 100 final instances with low variance
# low_variance_chunk = np.random.normal(50, 5, 100)
# data = pd.concat([data, pd.DataFrame({'Instance': range(len(data), len(data) + 100), 'Value': low_variance_chunk})], ignore_index=True)

# Save the dataset to a CSV file
data.to_csv('variance_dataset_low_100.csv', index=False)
