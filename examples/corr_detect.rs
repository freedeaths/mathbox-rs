use clap::Parser;
use csv::Reader;
//use plotters::prelude::*;
use std::error::Error;

use mathbox::stats::estimator::pcc;

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

/*
fn plot(outfile: &str, name: &str, signal: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
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
*/

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut rdr = Reader::from_path(cli.file.as_str())?;

    let obj = rdr
        .records()
        .map(|r| r.unwrap().get(3).unwrap().parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let mut top_candidate: Vec<(isize, f64)> = vec![];
    let num = 45; // rdr.records().count();
    for i in 0..num {
        rdr = Reader::from_path(cli.file.as_str())?; // ugly
        let candidate = rdr
            .records()
            .map(|r| r.unwrap().get(i).unwrap().parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let res = pcc(&obj, &candidate, 20).iter().fold((0, 0.0), |max, x| {
            if x.1 > max.1 {
                (x.0, x.1)
            } else {
                max
            }
        });
        top_candidate.push(res);
    }

    //top_candidate.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //top_candidate.reverse();
    //println!("{:?}", top_candidate);
    let mut result: Vec<(usize, (isize, f64))> = vec![];
    top_candidate.iter().enumerate().map(|(i, x)| result.push((i, *x))).count();
    result.sort_by(|a, b| a.1 .1.partial_cmp(&b.1 .1).unwrap());
    result.reverse();
    println!("{:?}", result);

    Ok(())
}
