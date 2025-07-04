use peroxide::fuga::*;
use puruspe::Inu_Knu;
use rustfft::{num_complex::Complex, FftPlanner};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Gaussian Random Fields using circulant embedding method 1
///
/// * Reference: Chan, Grace., An Effective Method for Simulating Gaussian Random Fields (1999)
pub fn grf(n: usize, kernel: Kernel) -> Vec<f64> {
    // Calculate the power of 2 greater than or equal to 2(n-1)
    let g = (2f64 * (n - 1) as f64).log2().ceil() as i32;
    let mut m = 2f64.powi(g) as usize;

    // Perform circulant embedding until a valid embedding is found
    let qa = loop {
        let c = circulant_embedding(m, n, |dx| kernel.eval(dx));
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(m);
        let mut c_fft = c.iter().map(|x| Complex::new(*x, 0f64)).collect::<Vec<_>>();
        fft.process(&mut c_fft);
        let c_fft = c_fft.iter().map(|x| x.re).collect::<Vec<_>>();
        let c_min = c_fft
            .iter()
            .min_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap();

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
    let mut a = z_fft
        .into_iter()
        .zip(qa)
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>();

    // Perform forward FFT on the result
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(m);
    fft.process(&mut a);

    // Extract the real part of the FFT result
    let y = a.iter().map(|x| x.re).collect::<Vec<_>>();

    // Return the first n elements of the result
    y[..n].to_vec()
}

/// Gaussian Random Fields using circulant embedding method 1 with rng
///
/// * Reference: Chan, Grace., An Effective Method for Simulating Gaussian Random Fields (1999)
///
/// # Example
/// ```
/// use peroxide::fuga::*;
/// use rugfield::{grf_with_rng, Kernel::SquaredExponential};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let n = 100;
///     let kernel = SquaredExponential(0.1);
///     let mut rng = stdrng_from_seed(42);
///     let grf_data = grf_with_rng(&mut rng, n, kernel);
///     // ...
///
///     Ok(())
/// }
/// ```
pub fn grf_with_rng<R: Rng + Clone>(rng: &mut R, n: usize, kernel: Kernel) -> Vec<f64> {
    // Calculate the power of 2 greater than or equal to 2(n-1)
    let g = (2f64 * (n - 1) as f64).log2().ceil() as i32;
    let mut m = 2f64.powi(g) as usize;

    // Perform circulant embedding until a valid embedding is found
    let qa = loop {
        let c = circulant_embedding(m, n, |dx| kernel.eval(dx));
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(m);
        let mut c_fft = c.iter().map(|x| Complex::new(*x, 0f64)).collect::<Vec<_>>();
        fft.process(&mut c_fft);
        let c_fft = c_fft.iter().map(|x| x.re).collect::<Vec<_>>();
        let c_min = c_fft
            .iter()
            .min_by(|&x, &y| x.partial_cmp(y).unwrap())
            .unwrap();

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
    let z = normal.sample_with_rng(rng, m);

    // Perform inverse FFT on the random samples
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(m);
    let mut z_fft = z.iter().map(|x| Complex::new(*x, 0f64)).collect::<Vec<_>>();
    fft.process(&mut z_fft);
    z_fft.iter_mut().for_each(|x| *x /= m as f64);

    // Multiply the inverse FFT result with the square root of the circulant embedding
    let mut a = z_fft
        .into_iter()
        .zip(qa)
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>();

    // Perform forward FFT on the result
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(m);
    fft.process(&mut a);

    // Extract the real part of the FFT result
    let y = a.iter().map(|x| x.re).collect::<Vec<_>>();

    // Return the first n elements of the result
    y[..n].to_vec()
}

/// Circulant embedding function
pub fn circulant_embedding<F: Fn(f64) -> f64>(m: usize, n: usize, kernel: F) -> Vec<f64> {
    let mut c = vec![0f64; m];
    let mid = m / 2;

    // Compute the first half of the circulant embedding using the provided kernel function
    for i in 0..mid + 1 {
        c[i] = kernel(i as f64 / n as f64);
    }
    // Mirror the first half to complete the circulant embedding
    for i in mid + 1..m {
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

/// Kernel type
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Kernel {
    /// Squared exponential kernel
    ///
    /// # Parameters
    /// - `l`: Length scale
    ///
    /// **Caution**: for the circulant embedding, the length scale is corresponded to the range (0,1)
    SquaredExponential(f64),
    /// Matern kernel
    ///
    /// # Parameters
    /// - `nu`: Smoothness parameter
    /// - `rho`: Length scale
    ///
    /// **Caution**: for the circulant embedding, the length scale is corresponded to the range (0,1)
    Matern(f64, f64),
    /// Locally Periodic kernel
    ///
    /// # Parameters
    /// - `p`: Period
    /// - `l`: Length scale
    LocalPeriodic(f64, f64),
    /// Rational Quadratic kernel
    ///
    /// # Parameters
    /// - `alpha`: Smoothness parameter
    /// - `l`: Length scale
    RationalQuadratic(f64, f64),
}

impl Kernel {
    pub fn eval(&self, dx: f64) -> f64 {
        match self {
            Kernel::SquaredExponential(l) => squared_exponential(dx, *l),
            Kernel::Matern(nu, rho) => matern(dx, *nu, *rho),
            Kernel::LocalPeriodic(p, l) => periodic(dx, *p, *l) * squared_exponential(dx, *l),
            Kernel::RationalQuadratic(alpha, l) => rational_quadratic(dx, *alpha, *l),
        }
    }
}

/// Squared exponential kernel
pub fn squared_exponential(dx: f64, l: f64) -> f64 {
    (-dx.powi(2) / (2.0 * l.powi(2))).exp()
}

/// Matern kernel
pub fn matern(dx: f64, nu: f64, rho: f64) -> f64 {
    let sqrt_2_nu = (2.0 * nu).sqrt();
    let mut sqrt_2_nu_dx_rho = sqrt_2_nu * dx.abs() / rho;
    if sqrt_2_nu_dx_rho == 0f64 {
        sqrt_2_nu_dx_rho = 1e-6;
    }
    let (_, knu) = Inu_Knu(nu, sqrt_2_nu_dx_rho);
    2f64.powf(1f64 - nu) / gamma(nu) * (sqrt_2_nu_dx_rho).powf(nu) * knu
}

/// Periodic kernel
pub fn periodic(dx: f64, p: f64, l: f64) -> f64 {
    (-(2f64 * (std::f64::consts::PI * dx.abs() / p).sin().powi(2)) / l.powi(2)).exp()
}

/// Rational Quadratic kernel
pub fn rational_quadratic(dx: f64, alpha: f64, l: f64) -> f64 {
    (1.0 + dx.powi(2) / (2.0 * alpha * l.powi(2))).powf(-alpha)
}
