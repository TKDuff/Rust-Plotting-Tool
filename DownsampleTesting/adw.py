import pandas as pd
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

    for y in y_vals:
        adwin.add(y)
        current_window_size = len(adwin.window)
        print(f"Current window size: {current_window_size}")

if __name__ == "__main__":
    main()
