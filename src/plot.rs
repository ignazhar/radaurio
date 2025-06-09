use std::io::Write;

use plotters::prelude::*;

// https://github.com/plotters-rs/plotters/blob/master/plotters/examples/area-chart.rs
const OUT_FILE_NAME: &str = "plotters-doc-data/frequency-chart-try10.png";
pub fn plot(data: Vec<f64>, caption: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut max_y: f64 = 0.0;
    for val in &data {
        max_y = max_y.max(*val);
    }

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(0..(data.len() - 1), (0.0..max_y).log_scale())?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter()).map(|(x, y)| (x, *y)),
            0.0,
            BLUE.mix(0.2),
        )
        .border_style(BLUE),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    // let aboba_test = 

    Ok(())
}

// https://github.com/plotters-rs/plotters/blob/master/plotters/examples/animation.rs
const OUT_FILE_NAME_GIF: &str = "plotters-doc-data/animation12.gif";
pub fn gif_plots(data: Vec<Vec<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(OUT_FILE_NAME_GIF, (800, 600), 50)?.into_drawing_area();

    let mut max_y: f64 = 0.0;
    for data_segment in &data {
        for val in data_segment {
            max_y = max_y.max(*val);
        }
    }

    let mut iter_counter: usize = 0;
    for data_segment in data {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .caption(format!("Frequency Chart Gif [iter #{}]", iter_counter), ("sans-serif", 40))
            .build_cartesian_2d((0..(data_segment.len() - 1)).log_scale(), 0.0..max_y)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        chart.draw_series(
            AreaSeries::new(
                (0..).zip(data_segment.iter()).map(|(x, y)| (x, *y)),
                0.0,
                RED.mix(0.2),
            )
            .border_style(RED),
        )?;
    
        root.present()?;
        print!(
            "\rFinished iteration #{}",
            iter_counter,
        );
        std::io::stdout().flush().unwrap();
        iter_counter += 1;
    }

    println!("Result has been saved to {}", OUT_FILE_NAME_GIF);

    Ok(())
}