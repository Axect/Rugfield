use rustfft::{FftPlanner, num_complex::Complex};
use peroxide::fuga::*;

/// Generate Gaussian Random Fields for a given range
pub fn gen_grf(x_min: f64, x_max: f64, sigma: f64, n: usize) -> Vec<f64> {
    let x_range = x_max - x_min;
    let sigma_new = sigma / x_range;
    grf(n, sigma_new)
}

/// Gaussian Random Fields using circulant embedding method 1
///
/// * Reference: Chan, Grace., An Effective Method for Simulating Gaussian Random Fields (1999)
fn grf(n: usize, sigma: f64) -> Vec<f64> {
    // Calculate the power of 2 greater than or equal to 2(n-1)
    let g = (2f64 * (n - 1) as f64).log2().ceil() as i32;
    let mut m = 2f64.powi(g) as usize;

    // Perform circulant embedding until a valid embedding is found
    let qa = loop {
        let c = circulant_embedding(m, n, |x| gaussian_kernel(x, sigma));
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(m);
        let mut c_fft = c.iter().map(|x| Complex::new(*x, 0f64)).collect::<Vec<_>>();
        fft.process(&mut c_fft);
        let c_fft = c_fft.iter().map(|x| x.re).collect::<Vec<_>>();
        let c_min = c_fft.iter().min_by(|&x, &y| x.partial_cmp(y).unwrap()).unwrap();

        if c_min >= &0f64 {
            break c_fft.fmap(|t| t.sqrt());
        } else if c_min.abs() < 1e-6 {
            break c_fft.fmap(|t| trunc(t).sqrt());
        } else {
            m *= 2;
        }
    };

    // Generate random samples from a standard normal distribution
    let normal = Normal(0f64, 1f64);
    let z = normal.sample(m);

    // Perform inverse FFT on the random samples
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(m);
    let mut z_fft = z.iter().map(|x| Complex::new(*x, 0f64)).collect::<Vec<_>>();
    fft.process(&mut z_fft);
    z_fft.iter_mut().for_each(|x| *x /= m as f64);

    // Multiply the inverse FFT result with the square root of the circulant embedding
    let mut a = z_fft.into_iter()
        .zip(qa)
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>();

    // Perform forward FFT on the result
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(m);
    fft.process(&mut a);

    // Extract the real part of the FFT result
    let y = a
        .iter()
        .map(|x| x.re)
        .collect::<Vec<_>>();

    // Return the first n elements of the result
    y[..n].to_vec()
}

/// Circulant embedding function
pub fn circulant_embedding<F: Fn(f64) -> f64>(m: usize, n: usize, kernel: F) -> Vec<f64> {
    let mut c = vec![0f64; m];
    let mid = m / 2;

    // Compute the first half of the circulant embedding using the provided kernel function
    for i in 0 .. mid + 1 {
        c[i] = kernel(i as f64 / n as f64);
    }
    // Mirror the first half to complete the circulant embedding
    for i in mid + 1 .. m {
        c[i] = c[m - i];
    }
    c
}

/// Truncation function
pub fn trunc(x: f64) -> f64 {
    if x < 0f64 {
        0f64
    } else {
        x
    }
}

/// Stationary Gaussian Kernel
pub fn gaussian_kernel(dx: f64, sigma: f64) -> f64 {
    (-dx.powi(2) / (2.0 * sigma.powi(2))).exp()
}
