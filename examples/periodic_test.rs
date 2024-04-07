use peroxide::fuga::*;
use rugfield::{grf, Kernel::LocalPeriodic};

fn main() -> Result<(), Box<dyn Error>> {
    let x_max = 100.0;
    let x_min = 0.0;
    let p = 1.0;
    let l = 0.8;
    let n = 1000;
    let samples = 8;

    let kernel = LocalPeriodic(p, l);

    let x = linspace_with_precision(x_min, x_max, n, 2);
    let grfs = (0 .. samples).map(|_| grf(n, kernel)).collect::<Vec<_>>();

    // Plot
    let line_style_cands = vec![LineStyle::Solid, LineStyle::Dashed, LineStyle::Dotted, LineStyle::DashDot];
    let color_cands = ["darkblue", "red", "darkgreen", "darkorange", "purple"];
    let mut line_style = vec![];
    let mut color = vec![];
    for i in 0 .. samples {
        line_style.push((i, line_style_cands[i % line_style_cands.len()]));
        color.push((i, color_cands[i % color_cands.len()]));
    }

    let mut plt = Plot2D::new();
    plt.set_domain(x);
    for grf in grfs.into_iter() {
        plt.insert_image(grf);
    }
    plt
        .set_line_style(line_style)
        .set_color(color)
        .set_xlabel(r"$x$")
        .set_ylabel(r"$y$")
        .tight_layout()
        .set_style(PlotStyle::Nature)
        .set_dpi(600)
        .set_path("examples/assets/periodic_test.png")
        .savefig()?;

    Ok(())
}
