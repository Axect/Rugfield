use rugfield::gen_grf;
use peroxide::fuga::*;

fn main() -> Result<(), Box<dyn Error>> {
    let x_max = 10.0;
    let x_min = 0.0;
    let sigma = 1.0;
    let n = 1000;
    let samples = 8;

    let x = linspace_with_precision(x_min, x_max, n, 2);
    let grfs = (0 .. samples).map(|_| gen_grf(x_min, x_max, sigma, n)).collect::<Vec<_>>();

    // Plot
    let line_style = [LineStyle::Solid, LineStyle::Dotted, LineStyle::Dashed, LineStyle::DashDot];
    let line_style = line_style.iter().cycle().take(samples).cloned().collect::<Vec<_>>();
    let color = ["darkblue", "red", "darkgreen", "darkorange", "purple"];
    let color = color.iter().cycle().take(samples).cloned().collect::<Vec<_>>();

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
        .set_path("examples/simple.png")
        .savefig()?;

    Ok(())
}
