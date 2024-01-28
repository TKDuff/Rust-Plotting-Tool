import pandas as pd
import matplotlib.pyplot as plt
import time
import math

class ADWIN:
    def __init__(self, delta=0.000000000000000000001):
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

    def get_aggregated_point(self):
        return sum(self.window) / len(self.window) if self.window else None

def read_data(file_path):
    return pd.read_csv(file_path)

def plot_live(x, y, aggregated_points):
    plt.ion()
    plt.clf()
    plt.plot(x, y, label='Data Stream')  # Label for the data stream
    if aggregated_points:
        agg_x, agg_y = zip(*aggregated_points)
        plt.plot(agg_x, agg_y, color='red', label='Aggregated Trend', linestyle='-')
    plt.xlabel('X-axis')
    plt.ylabel('Y-axis')
    if x and y:  # Only call legend if there is data to plot
        plt.legend()
    plt.draw()
    plt.pause(0.001)



def main():
    file = 'variance_dataset.csv'
    file_path = "/home/thomas/FinalYearProject/online-graph/DownsampleTesting/plot_data/%s" % (file)
    df = read_data(file_path)
    adwin = ADWIN(delta=0.000000000000000000001)

    aggregated_points = []
    x_vals, y_vals = [], []

    for index, row in df.iterrows():
        y = row['y_col']  # Assuming y_col is the column to process
        adwin.add(y)
        x_vals.append(index)
        y_vals.append(y)

        if len(adwin.window) == 1:  # New window started, aggregate previous window
            aggregated_point = adwin.get_aggregated_point()
            if aggregated_point is not None:
                aggregated_points.append((index-1, aggregated_point))

        plot_live(x_vals, y_vals, aggregated_points)
        #time.sleep(0.2)  # Delay to simulate streaming

    plt.ioff()
    plt.show()

if __name__ == "__main__":
    main()
