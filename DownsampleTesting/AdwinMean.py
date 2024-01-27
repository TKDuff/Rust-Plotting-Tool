import pandas as pd
import matplotlib.pyplot as plt
import math

class ADWIN:
    def __init__(self, delta=0.002):
        self.delta = delta
        self.window = []

    def add(self, value):
        self.window.append(value)
        self.cut_window()

    def cut_window(self):
        while len(self.window) > 1 and self.check_cut():
            self.window.pop(0)

    def check_cut(self):
        for i in range(1, len(self.window)):
            mean1 = sum(self.window[:i]) / i
            mean2 = sum(self.window[i:]) / (len(self.window) - i)
            n1 = i
            n2 = len(self.window) - i
            epsilon = math.sqrt(1 / (2 * n1) * math.log(4 / self.delta)) + math.sqrt(1 / (2 * n2) * math.log(4 / self.delta))
            if abs(mean1 - mean2) > epsilon:
                return True
        return False

# Function to read data from CSV
def read_data(file_path):
    df = pd.read_csv(file_path)
    return df['x_col'], df['y_col']

def plot_window_means(x_vals, y_vals, window_end_indices):
    plt.figure(figsize=(12, 6))
    
    # Convert pandas Series to numpy arrays
    x_vals_np = x_vals.to_numpy()
    y_vals_np = y_vals.to_numpy()

    # Initialize start index for the first window
    start_index = 0

    # Lists to store the mean of each window
    window_means_x = []
    window_means_y = []

    # Calculate means for each window
    for end_index in window_end_indices:
        if start_index <= end_index:
            # Calculate the mean for the current window
            window_x_vals = x_vals_np[start_index:end_index+1]
            window_y_vals = y_vals_np[start_index:end_index+1]
            mean_x = window_x_vals.mean()
            mean_y = window_y_vals.mean()
            
            # Store the means in their respective lists
            window_means_x.append(mean_x)
            window_means_y.append(mean_y)
            
            # Update start_index for the next window
            start_index = end_index + 1

    # Plot the original data stream
    plt.plot(x_vals_np, y_vals_np, label='Data Stream', alpha=0.5)

    # Plot the means as a line plot to connect them
    plt.plot(window_means_x, window_means_y, color='red', label='Window Means')

    # Title and labels
    plt.title('Means of Windows in Data Stream')
    plt.xlabel('X-axis')
    plt.ylabel('Y-axis')
    plt.legend()
    plt.show()

# ... (main function remains unchanged) ...

# Ensure the main function calls the new plot function
def main():
    #file = 'variance_dataset_low_100.csv'
    file = 'variance_dataset.csv'
    file_path = "/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file)
    x_vals, y_vals = read_data(file_path)

    adwin = ADWIN(delta=0.00000000000000001)
    window_end_indices = []

    for i, y in enumerate(y_vals):
        adwin.add(y)
        if len(adwin.window) == 1:  # New window started
            window_end_indices.append(i)

    plot_window_means(x_vals, y_vals, window_end_indices)

if __name__ == "__main__":
    main()
