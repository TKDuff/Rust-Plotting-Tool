import csv
import random

def generate_bin(base_mean, max_noise=1.0):
    # Add more noise within the bounds to ensure mean increases but not too much
    noise = random.uniform(-max_noise, max_noise)
    mean = base_mean + noise

    # Set min and max values around the mean
    min_val = mean - 5
    max_val = mean + 5

    # Simplified Sum of Squares
    sos = mean ** 2
    return mean, min_val, max_val, sos

def create_valley(mean, i, num_bins):
    # Create two valleys at different intervals
    if num_bins // 4 < i < num_bins // 3 or 2 * num_bins // 3 < i < 3 * num_bins // 4:
        return mean - 10  # Lower the mean for valleys
    return mean

def write_bins_to_csv(filename, num_bins=100, start_mean=10, mean_increment=2, max_noise=1.0):
    with open(filename, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow(['mean_x', 'min_x', 'max_x', 'sos_x', 'mean_y', 'min_y', 'max_y', 'sos_y'])

        for i in range(num_bins):
            x_mean = start_mean + i * mean_increment
            y_mean = create_valley(start_mean + i * mean_increment, i, num_bins)  # Apply valley effect to y

            bin_x = generate_bin(x_mean, max_noise)
            bin_y = generate_bin(y_mean, max_noise)

            writer.writerow(bin_x + bin_y)

# Usage
write_bins_to_csv('mock_bins_data.csv')
