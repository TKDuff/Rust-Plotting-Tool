import numpy as np

# Example data (replace this with your actual bin data)
bin_data = [np.random.normal(loc, 1, size) for loc, size in zip([10, 15, 20, 25], [20, 5, 40, 35])]
overall_mean = np.mean(np.concatenate(bin_data))

# Calculate weighted variance
weights = [len(bin) for bin in bin_data]
variances = [np.var(bin, ddof=1) for bin in bin_data]
means = [np.mean(bin) for bin in bin_data]

combined_variance = sum(weight * (variance + (mean - overall_mean)**2)
                        for weight, variance, mean in zip(weights, variances, means)) / sum(weights)

# Compare with overall variance
overall_variance = np.var(np.concatenate(bin_data), ddof=1)

print("Variance of Full Dataset:", overall_variance)
print("Combined Variance:", combined_variance)
print("Are they close:", np.isclose(overall_variance, combined_variance))