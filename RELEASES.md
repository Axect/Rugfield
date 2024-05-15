# Ver 0.2.2 (2024-05-15)

- Implement `grf_with_rng` function
- Bump `peroxide` to `0.37.3`

# Ver 0.2.1 (2024-04-07)

- Implement more kernels
  - `LocalPeriodic`
  - `RationalQuadratic`

# Ver 0.2.0 (2024-04-06)

- Remove `gen_grf` function
- Implement Matern kernel
  - To implement Matern kernel, add `puruspe` crate as dependency (for Bessel function)
- Define `Kernel` enum
  - `SquaredExponential(f64)`
  - `Matern(f64, f64)`
- Change `grf` arguments (now, requires kernel enum)
- Add `serde` feature

# Ver 0.1.0

- Initial release
- Implement Squared Exponential kernel
- Implement circulant embedding method for efficient GRF generation
- Utilize `rustfft` library for Fast Fourier Transform (FFT) operations
- Provide stationary Gaussian kernel function
