use plotters::prelude::*;
use std::error::Error;

//given a set of values plot the histogram
pub fn histogram(data: &Vec<usize>, output: &str, 
    xmax: usize, ymax: usize,
    xtitle: &str, ytitle: &str,
    title: &str,
    clip: usize) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    
    //initilalize a chart
    let mut chart = ChartBuilder::on(&root)
    .caption(title, ("sans-serif", 20).into_font())
    .x_label_area_size(35)
    .y_label_area_size(40)
    .margin(40)
    .build_cartesian_2d(0..xmax, 0..ymax)?;
    
    //configure the x-y plain
    chart.configure_mesh()
    .x_labels(20) 
    .y_labels(20)
    .x_label_style(("sans-serif", 20)) 
    .y_label_style(("sans-serif", 20)) 
    .x_desc(xtitle)
    .y_desc(ytitle) 
    .axis_desc_style(("sans-serif", 15))
    .draw()?;
    
    //plot the histogram
    let clipped_data: Vec<usize> = data.iter().filter(|&&x| x < clip as usize).cloned().collect();
    chart.draw_series(
    Histogram::vertical(&chart)
    .style(RED.filled())
    .data(clipped_data.iter().map(|x| (*x, 1)))
    )?;

    //present the chart
    root.present()?;
    Ok(())
    }