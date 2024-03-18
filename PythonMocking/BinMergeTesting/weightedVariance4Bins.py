import numpy as np

# Creating an array of 100 points with decimals
# data = np.array([
#     25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
#         25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
# ]).round(3)

data = np.array([
    12.345, 23.456, 34.567, 45.678, 56.789, 67.890, 78.901, 89.012, 90.123, 1.234,
     2.345, 3.456, 4.567, 5.678, 6.789, 7.890, 8.901, 9.012, 10.123, 11.234,
     12.345, 13.456, 14.567, 15.678, 16.789, 17.890, 18.901, 19.012, 20.123, 21.234,
     22.345, 23.456, 24.567, 25.678, 26.789, 27.890, 28.901, 29.012, 30.123, 31.234,
     32.345, 33.456, 34.567, 35.678, 36.789, 37.890, 38.901, 39.012, 40.123, 41.234,
     42.345, 43.456, 44.567, 45.678, 46.789, 47.890, 48.901, 49.012, 50.123, 51.234,
     52.345, 53.456, 54.567, 55.678, 56.789, 57.890, 58.901, 59.012, 60.123, 61.234,
     62.345, 63.456, 64.567, 65.678, 66.789, 67.890, 68.901, 69.012, 70.123, 71.234,
     72.345, 73.456, 74.567, 75.678, 76.789, 77.890, 78.901, 79.012, 80.123, 81.234,
     82.345, 83.456, 84.567, 85.678, 86.789, 87.890, 88.901, 89.012, 90.123, 91.234
]).round(3)

# Calculate the overall mean and variance of the 100 points
overall_mean = data.mean()
overall_variance = np.var(data, ddof=1)

# Creating sub-arrays with specified sizes
bin1 = data[:20]         # First 20 points
bin2 = data[20:25]       # Next 5 points
bin3 = data[25:65]       # Next 40 points
bin4 = data[65:]         # Last 35 points

# Calculate variance, mean, count, and sum for each bin
bin_stats = [(np.var(bin, ddof=1), np.mean(bin), len(bin), np.sum(bin)) for bin in [bin1, bin2, bin3, bin4]]

# Calculate the combined variance using the formula
N = sum([count for _, _, count, _ in bin_stats])
combined_mean = sum([mean * count for _, mean, count, _ in bin_stats]) / N
combined_variance = sum([(count - 1) * variance + count * (mean - combined_mean) ** 2 for variance, mean, count, _ in bin_stats]) / N

# Checking if the combined variance is close to the original variance
print("Variance of Full Dataset:", overall_variance)
print("Combined Variance:", combined_variance)
print("Are they close:", np.isclose(overall_variance, combined_variance))
