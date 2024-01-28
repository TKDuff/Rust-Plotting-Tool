import pandas as pd
import matplotlib.pyplot as plt
import math

class ADWIN:
    def __init__(self, delta=0.00000000000000001):
        self.delta = delta
        self.window = []
        self.index = 0  # Add an index attribute to track the position in the data stream

    def add(self, value):
        self.window.append(value)
        self.index += 1  # Increment the index counter here
        self.cut_window()

    def cut_window(self):
        cut_index = self.check_cut()
        if cut_index is not None:
            # Calculate the mean of the segment being removed
            mean_of_cut_segment = sum(self.window[:cut_index]) / cut_index
            # Store the index and the mean value in the window_end_indices list
            window_end_indices.append((self.index - len(self.window) + cut_index, mean_of_cut_segment))
            # Remove the segment from the window
            del self.window[:cut_index]

    def check_cut(self):
        for i in range(1, len(self.window)):
            mean1 = sum(self.window[:i]) / i
            mean2 = sum(self.window[i:]) / (len(self.window) - i)
            n1 = i
            n2 = len(self.window) - i
            epsilon = math.sqrt(1 / (2 * n1) * math.log(4 / self.delta)) + math.sqrt(1 / (2 * n2) * math.log(4 / self.delta))
            if abs(mean1 - mean2) > epsilon:
                return i  # Return the index where the cut should occur
        return None

# Function to read data from CSV
def read_data(file_path):
    df = pd.read_csv(file_path)
    return df['x_col'], df['y_col']

def plot_window_means(x_vals, y_vals, window_end_indices):
    plt.figure(figsize=(12, 6))
    x_vals_np = x_vals.to_numpy()
    y_vals_np = y_vals.to_numpy()

    # Plot the original data stream
    plt.plot(x_vals_np, y_vals_np, label='Data Stream', alpha=0.5)

    # Prepare x and y values for the aggregate points line plot
    agg_x_vals = [index for index, _ in window_end_indices]
    agg_y_vals = [mean for _, mean in window_end_indices]

    # Plot the means as a line plot to connect them
    plt.plot(agg_x_vals, agg_y_vals, color='red', label='Aggregated Means')

    plt.title('Means of Cut Segments in Data Stream')
    plt.xlabel('X-axis')
    plt.ylabel('Y-axis')
    plt.legend()
    plt.show()

# Main function
def main():
    #file = 'variance_dataset.csv'
    #file = 'variance_dataset_low_100.csv'
    file = 'Generated_Reward_Data_10000.csv'
    file_path = "/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file)
    x_vals, y_vals = read_data(file_path)

    adwin = ADWIN(delta=0.00000000000000001)
    global window_end_indices
    window_end_indices = []  # Store indices and means of cut segments

    for i, y in enumerate(y_vals):
        adwin.add(y)

    plot_window_means(x_vals, y_vals, window_end_indices)

if __name__ == "__main__":
    main()
