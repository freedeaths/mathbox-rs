use clap::Parser;
use csv::Reader;
use std::error::Error;
use plotters::prelude::*;

use mathbox::app::signal::filter::{moving_median, dft_filter_lowpass};
use mathbox::app::signal::outlier::normal_outlier;

#[derive(Parser, Debug)]
#[clap(name = "decompose")]
#[clap(author = "Freedeaths")]
#[clap(version = "0.1")]
#[clap(about = "Decompose a time series signal", long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long)]
    file: String,
}

fn plot(outfile: &str, name: &str, signal:&[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let y_max = signal.iter().cloned().fold(f64::NAN, f64::max);
    let y_min: f64 = signal.iter().cloned().fold(f64::NAN, f64::min);
    let root = BitMapBackend::new(outfile, (640, 240)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(name, ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0u32..800u32, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            signal.iter().enumerate().map(|(i, x)| (i as u32, *x as f64)),
            &RED,
        ))?
        .label(name)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut rdr = Reader::from_path(cli.file.as_str())?;

    let signal = rdr.records().map(|r| r.unwrap().get(1).unwrap().parse::<f64>().unwrap()).collect::<Vec<_>>();
    plot("origin.png", "origin", &signal)?;

    let trend = moving_median(&signal, 7);
    plot("trend.png", "trend", &trend)?;

    let moved_trand = signal.iter().zip(trend.iter()).map(|(x, y)| x - y).collect::<Vec<_>>();
    let (_, seasonality) =dft_filter_lowpass(&moved_trand, 86400.0, 0.015/86400.0, 3);
    plot("seasonality.png", "seasonality", &seasonality)?;

    let noise = moved_trand.iter().zip(seasonality.iter()).map(|(x, y)| x - y).collect::<Vec<_>>();
    plot("noise.png", "noise", &noise)?;
    let outlier = normal_outlier(&noise, 3.0);
    println!("outliers: {:?}", outlier);

    Ok(())
}
