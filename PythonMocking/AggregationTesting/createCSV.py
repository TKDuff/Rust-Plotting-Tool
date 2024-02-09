import csv

# Generate x and y data
x_data = list(range(1, 101))  # Generate x values from 1 to 100
y_data = [2 * x + 5 for x in x_data]  # Generate y values as a straight line y = 2x + 5

# Add a big step to y_data
step_size = 100  # Size of the step
for i in range(100, 150):
    y_data.append(y_data[-1] + step_size)

# Add fluctuations to y_data
import random

for i in range(150, 201):
    y_data.append(y_data[-1] + random.uniform(-5, 5))

# Write the data to a CSV file
with open('data.csv', 'w', newline='') as csvfile:
    fieldnames = ['x', 'y']
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
    
    writer.writeheader()
    for x, y in zip(x_data, y_data):
        writer.writerow({'x': x, 'y': y})

print("CSV file 'data.csv' created successfully.")
