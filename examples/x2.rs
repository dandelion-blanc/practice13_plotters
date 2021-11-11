
extern crate plotters;

use plotters::prelude::*;


fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let root = BitMapBackend::new("./x2.png", (1280, 960)).into_drawing_area();
	root.fill(&WHITE)?;

	let mut chart = ChartBuilder::on(&root)
							.caption("y=x^2", ("Times New Roman", 50).into_font())
							.margin(30)
							.x_label_area_size(60)
							.y_label_area_size(80)
							. build_cartesian_2d(-1.0_f32..1.0_f32, -0.1_f32..1.0_f32)?;

	chart.configure_mesh()
			.axis_desc_style(("Times New Roman", 40))
			.x_desc("x")
			.y_desc("y")
			.label_style(("Times New Roman", 25))
			.draw()?;
	chart.draw_series(LineSeries::new(
								(-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
								&RED,
							))?
		.label("y = x^2")
		.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

