import pandas as pd
import matplotlib.pyplot as plt
import math

# Simple ADWIN Implementation
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

    def get_mean(self):
        return sum(self.window) / len(self.window) if self.window else 0.0

# Function to read data from CSV
def read_data(file_path):
    df = pd.read_csv(file_path)
    return df['x_col'], df['y_col']

# Main program
def main():
    file = 'variance_dataset.csv'
    file_path = "/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file)
    x_vals, y_vals = read_data(file_path)

    adwin = ADWIN(delta=0.002)

    means, window_sizes = [], []

    for y in y_vals:
        adwin.add(y)
        means.append(adwin.get_mean())
        window_sizes.append(len(adwin.window))

    # Convert to NumPy arrays for compatibility with matplotlib
    x_vals_np = x_vals.values
    y_vals_np = y_vals.values
    means_np = pd.Series(means).values
    window_sizes_np = pd.Series(window_sizes).values

    # Plotting
    plt.figure(figsize=(12, 6))
    plt.subplot(2, 1, 1)
    plt.plot(x_vals_np, y_vals_np, label='Data Stream')
    plt.plot(x_vals_np, means_np, label='ADWIN Mean', color='red')
    plt.title('Data Stream and ADWIN Mean')
    plt.legend()

    plt.subplot(2, 1, 2)
    plt.plot(x_vals_np, window_sizes_np, label='Window Size', color='green')
    plt.title('ADWIN Window Size')
    plt.legend()
    plt.show()


if __name__ == "__main__":
    main()


# import numpy as np
# import matplotlib.pyplot as plt

# class ADWIN:
#     def __init__(self, delta=0.002):
#         self.delta = delta
#         self.window = []

#     def add(self, value):
#         self.window.append(value)
#         self.cut_window()

#     def cut_window(self):
#         while len(self.window) > 1 and self.check_cut():
#             self.window.pop(0)

#     def check_cut(self):
#         for i in range(1, len(self.window)):
#             mean1 = sum(self.window[:i]) / i
#             mean2 = sum(self.window[i:]) / (len(self.window) - i)
#             n1 = i
#             n2 = len(self.window) - i
#             epsilon = np.sqrt(1 / (2 * n1) * np.log(4 / self.delta)) + np.sqrt(1 / (2 * n2) * np.log(4 / self.delta))
#             if abs(mean1 - mean2) > epsilon:
#                 return True
#         return False

#     def get_mean(self):
#         return sum(self.window) / len(self.window) if self.window else 0.0

# # Simulate a stream of data with changing distribution
# np.random.seed(0)
# data_stream = np.concatenate([
#     np.random.normal(150, 10, 50),  # Initial distribution
#     np.random.normal(450, 10, 50),  # Changed distribution
# ])

# adwin = ADWIN(delta=0.002)
# window_sizes = []

# for value in data_stream:
#     adwin.add(value)
#     window_sizes.append(len(adwin.window))

# # Plot the data and ADWIN's window size
# plt.figure(figsize=(12, 6))
# plt.subplot(2, 1, 1)
# plt.plot(data_stream, label='Data Stream')
# plt.title('Data Stream')

# plt.subplot(2, 1, 2)
# plt.plot(window_sizes, label='Window Size', color='green')
# plt.title('ADWIN Window Size')

# plt.tight_layout()
# plt.show()
