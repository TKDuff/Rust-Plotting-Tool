import csv
import random

# Number of points
num_points = 100000

# Variance parameters
low_variance_mean = 40
low_variance_std = 7
high_variance_mean = 47.5
high_variance_std = 20
high_variance_starts = [5000, 15000, 30000, 45000, 55000, 65000, 75000, 85000, 90000, 95000]
high_variance_ends = [8000, 20000, 35000, 48000, 58000, 68000, 78000, 88000, 92000, 98000]
dip_starts = [20000, 50000, 80000]
dip_ends = [25000, 60000, 90000]
dip_mins = [35, 25, 30]
dip_maxs = [45, 55, 40]

# Create data
data = []
for i in range(num_points):
    if any(start <= i < end for start, end in zip(high_variance_starts, high_variance_ends)):
        y_value = random.gauss(high_variance_mean, high_variance_std)
    elif any(start <= i < end for start, end in zip(dip_starts, dip_ends)):
        dip_index = dip_starts.index(next(start for start in dip_starts if start <= i < dip_ends[dip_starts.index(start)]))
        dip_progress = (i - dip_starts[dip_index]) / (dip_ends[dip_index] - dip_starts[dip_index])
        y_value = dip_mins[dip_index] + dip_progress * (dip_maxs[dip_index] - dip_mins[dip_index])
    else:
        y_value = random.gauss(low_variance_mean, low_variance_std)
    data.append((i, y_value))

# Write data to CSV file
with open('extended_variance_dataset_100000_more_variation.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['x_col', 'y_col'])
    writer.writerows(data)

print("CSV file 'extended_variance_dataset_100000_more_variation.csv' created successfully.")
