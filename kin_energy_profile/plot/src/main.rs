use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let pattern = "../di-vd_kin_energy_profile_at_time_*.csv";

    let df_di_vd = LazyCsvReader::new_paths(vec![pattern.into()].into())
        .with_has_header(true)
        .finish()?
        // 3. Gruppieren nach 'Bin' und Durchschnitt von 'Average' berechnen
        .group_by([col("Bin")])
        .agg([
            col("Average").mean().alias("Average"), // Berechnet den Mittelwert über alle Dateien
            col("Count").sum().alias("Total_Count") // Optional: Summe der Counts
        ])
        // 4. Sortieren nach Bin, damit der Plot später stimmt
        .sort(["Bin"], Default::default())
        .collect()?;

    let pattern = "../di_kin_energy_profile_at_time_*.csv";

    let df_di = LazyCsvReader::new_paths(vec![pattern.into()].into())
        .with_has_header(true)
        .finish()?
        // 3. Gruppieren nach 'Bin' und Durchschnitt von 'Average' berechnen
        .group_by([col("Bin")])
        .agg([
            col("Average").mean().alias("Average"), // Berechnet den Mittelwert über alle Dateien
            col("Count").sum().alias("Total_Count") // Optional: Summe der Counts
        ])
        // 4. Sortieren nach Bin, damit der Plot später stimmt
        .sort(["Bin"], Default::default())
        .collect()?;

    let pattern = "../se_kin_energy_profile_at_time_*.csv";

    let df_se = LazyCsvReader::new_paths(vec![pattern.into()].into())
        .with_has_header(true)
        .finish()?
        // 3. Gruppieren nach 'Bin' und Durchschnitt von 'Average' berechnen
        .group_by([col("Bin")])
        .agg([
            col("Average").mean().alias("Average"), // Berechnet den Mittelwert über alle Dateien
            col("Count").sum().alias("Total_Count") // Optional: Summe der Counts
        ])
        // 4. Sortieren nach Bin, damit der Plot später stimmt
        .sort(["Bin"], Default::default())
        .collect()?;

    println!("Kombinierter DataFrame:\n{:?}", df_di_vd);
    println!("Kombinierter DataFrame:\n{:?}", df_di);
    println!("Kombinierter DataFrame:\n{:?}", df_se);

    // 2. Daten extrahieren
    // Wir nutzen f64 für die Berechnung der Achsen-Limits
    let bins_di_vd: Vec<i64> = df_di_vd.column("Bin")?.i64()?.into_no_null_iter().collect();
    let averages_di_vd: Vec<f64> = df_di_vd.column("Average")?.f64()?.into_no_null_iter().collect();
    let bins_di: Vec<i64> = df_di.column("Bin")?.i64()?.into_no_null_iter().collect();
    let averages_di: Vec<f64> = df_di.column("Average")?.f64()?.into_no_null_iter().collect();
    let bins_se: Vec<i64> = df_se.column("Bin")?.i64()?.into_no_null_iter().collect();
    let averages_se: Vec<f64> = df_se.column("Average")?.f64()?.into_no_null_iter().collect();

    // Dynamische Limits für die Achsen bestimmen
    let x_max = *bins_di.iter().max().unwrap_or(&10) as f64;
    let y_max = averages_di.iter().copied().fold(f64::NAN, f64::max) * 1.1; // 10% Puffer oben

    let x_max2 = (*bins_di_vd.iter().max().unwrap_or(&10) as f64).max(x_max);
    let y_max2 = (averages_di_vd.iter().copied().fold(f64::NAN, f64::max) * 1.1).max(y_max); // 10% Puffer oben

    let x_max1 = (*bins_se.iter().max().unwrap_or(&10) as f64).max(x_max);
    let y_max1 = (averages_se.iter().copied().fold(f64::NAN, f64::max) * 1.1).max(y_max); // 10% Puffer oben

    // 3. Plotters Setup
    let root = BitMapBackend::new("kin_enery_plot1.png", (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
    .margin(50)
    .x_label_area_size(70)
    .y_label_area_size(100)
    // Hier i64 nutzen (0i64..x_max + 1)
    .build_cartesian_2d(0f64..(x_max1 + 1.), -1.0..y_max1)?;

    chart.configure_mesh()
        // .x_label_formatter(&|x| format!("{}", -20. + (*x as f32 / 10.)))
        // .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
        // .x_labels(15)
        // .y_labels(5)
        .label_style(("sans-serif", 30))
        .x_desc("Height z [m]")
        .y_desc("Kinetic energy [J]")
        .axis_desc_style(("sans-serif", 40))
        .draw()?;

    let points: Vec<(f64, f64)> = bins_di.clone().into_iter().zip(averages_di.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: RED.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Kinetic energy for IISPH with DI source term")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: RED.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    let points: Vec<(f64, f64)> = bins_se.clone().into_iter().zip(averages_se.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: BLUE.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Kinetic energy for SESPH")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: BLUE.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .label_font(("sans-serif", 30))
        .draw()?;

    root.present()?;

    // 3. Plotters Setup
    let root = BitMapBackend::new("kin_enery_plot2.png", (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
    .margin(50)
    .x_label_area_size(70)
    .y_label_area_size(100)
    // Hier i64 nutzen (0i64..x_max + 1)
    .build_cartesian_2d(0f64..(x_max2 + 1.), -0.01..y_max2)?;

    chart.configure_mesh()
        // .x_label_formatter(&|x| format!("{}", -20. + (*x as f32 / 10.)))
        // .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
        // .x_labels(15)
        // .y_labels(5)
        .label_style(("sans-serif", 30))
        .x_desc("Height z [m]")
        .y_desc("Kinetic energy [J]")
        .axis_desc_style(("sans-serif", 40))
        .draw()?;

    let points: Vec<(f64, f64)> = bins_di.clone().into_iter().zip(averages_di.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: RED.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Kinetic energy for IISPH with DI source term")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: RED.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    let points: Vec<(f64, f64)> = bins_di_vd.clone().into_iter().zip(averages_di_vd.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: BLUE.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Kinetic energy for IISPH with DI+VD source term")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: BLUE.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .label_font(("sans-serif", 30))
        .draw()?;

    root.present()?;

    println!("Plots wurden gespeichert.");
    Ok(())
}