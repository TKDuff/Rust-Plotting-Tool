import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.patches as patches
import math

# ADWIN Implementation
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

# Plotting function
def plot_with_windows(x_vals, y_vals, window_end_indices):
    plt.figure(figsize=(12, 6))

    # Convert pandas Series to numpy arrays
    x_vals_np = x_vals.to_numpy()
    y_vals_np = y_vals.to_numpy()

    plt.plot(x_vals_np, y_vals_np, label='Data Stream')
    
    # Add window rectangles
    start_index = 0
    for end_index in window_end_indices:
        if end_index < len(x_vals_np):
            window = patches.Rectangle((x_vals_np[start_index], min(y_vals_np)), 
                                       x_vals_np[end_index] - x_vals_np[start_index], 
                                       max(y_vals_np) - min(y_vals_np), 
                                       alpha=0.2, 
                                       color='grey',
                                       edgecolor='none')  # Set edgecolor to 'none'
            plt.gca().add_patch(window)
            start_index = end_index + 1

    plt.title('Data Stream with ADWIN Windows')
    plt.xlabel('X-axis')
    plt.ylabel('Y-axis')
    plt.legend()
    plt.show()


# Main program
def main():
    file = 'variance_dataset.csv'
    file_path = "/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file)

    adwin = ADWIN(delta=0.002)
    window_end_indices = []

    x_vals, y_vals = read_data(file_path)

    for i, y in enumerate(y_vals):
        adwin.add(y)
        if len(adwin.window) == 1:  # New window started
            window_end_indices.append(i - 1)

    plot_with_windows(x_vals, y_vals, window_end_indices)

if __name__ == "__main__":
    main()
