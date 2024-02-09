import csv
import matplotlib.pyplot as plt

def plot_means_from_csv(filename):
    x_means = []
    y_means = []

    with open(filename, mode='r') as file:
        reader = csv.reader(file)
        next(reader)  # Skip the header row
        for row in reader:
            x_mean, _, _, _, y_mean, _, _, _ = row
            x_means.append(float(x_mean))
            y_means.append(float(y_mean))

    plt.plot(x_means, y_means)  # Connect points with lines
    plt.xlabel('X Means')
    plt.ylabel('Y Means')
    plt.title('Plot of X and Y Means')
    plt.show()

# Usage
plot_means_from_csv('mock_bins_data.csv')
