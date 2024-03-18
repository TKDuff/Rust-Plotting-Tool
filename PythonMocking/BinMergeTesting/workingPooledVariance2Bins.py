import numpy as np

# 1) Creates an array of 50 points
np.random.seed(0)
data = np.array([
    25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
        25.0, 23.0, 24.0, 28.0, 29.0, 27.5, 23.5, 26.6, 27.9, 21.3,
]).round(3)

# 2) Calculate the overall mean and variance of the 50 points
overall_mean = np.mean(data)
overall_variance = np.var(data, ddof=1)

# 3) Creating sub-arrays: 10 elements, 25 elements, and 15 elements
bin1, bin2, bin3 = np.split(data, [10, 35])

# 4) Calculate variance, mean, and count for each bin
bin_stats = [(np.var(bin, ddof=1), np.mean(bin), len(bin)) for bin in [bin1, bin2, bin3]]

# 5) Calculating the combined variance using the provided formula
combined_variance = sum((count - 1) * variance + count * (mean - overall_mean) ** 2 
                        for variance, mean, count in bin_stats) / (sum(count for _, _, count in bin_stats) - 1)

# Checking if the combined variance is close to the original variance
print("Variance of Full Dataset:", overall_variance)
print("Combined Variance:", combined_variance)
print("Are they close:", np.isclose(overall_variance, combined_variance))
