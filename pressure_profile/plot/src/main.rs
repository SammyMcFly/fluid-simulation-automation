use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;
use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let pattern = "../di-vd_pressure_profile_at_time_*.csv";

    let mut lfs = Vec::new();
    // 1. Jede Datei einzeln scannen und Spalten umbenennen
    for (i, entry) in glob(pattern).expect("Failed to read glob pattern").enumerate() {
        let path = entry.expect("Invalid path");
        let file_name = path.file_stem().unwrap().to_string_lossy();

        let lf = LazyCsvReader::new(path)
            .with_has_header(true)
            .finish()?
            .select([
                col("Bin"),
                // "Average" wird zu "Average_Datei1", "Average_Datei2", etc.
                col("Average").alias(&format!("Average_{}", i)),
            ]);

        lfs.push(lf);
    }
    // 2. Alle Dateien über die "Bin"-Spalte horizontal zusammenführen
    let mut combined_lf = lfs.remove(0);
    for next_lf in lfs {
        combined_lf = combined_lf.left_join(next_lf, col("Bin"), col("Bin"));
    }

    let n_files = 10; // Beispielhaft für deine Dateien
    let avg_cols: Vec<Expr> = (0..n_files)
        .map(|i| col(&format!("Average_{}", i)))
        .collect();
    // 2. Mittelwert und Standardabweichung horizontal berechnen
    let df_di_vd = combined_lf
        .with_columns([
            avg_cols.iter().cloned()
                .fold(lit(0.0), |acc, x| acc + x)
                .apply(move |s| Ok(Some(s / (n_files) as f64)), GetOutput::from_type(DataType::Float64))
                .alias("Bin_Mean")
        ])
        .with_columns([
            // Horizontale Standardabweichung (Manuelle Formel: sqrt(mean(x^2) - mean(x)^2))
            // Wir berechnen die Summe der quadrierten Abweichungen
            avg_cols.iter().cloned()
                .map(|c| (c - col("Bin_Mean")).pow(2))
                .fold(lit(0.0), |acc, x| acc + x)
                .apply(move |s| Ok(Some(s / ((n_files - 1) as f64))), GetOutput::from_type(DataType::Float64))
                .alias("Bin_Variance")
        ])
        .collect()?;
    let df_di_vd = df_di_vd
    .lazy()
    .with_column(
        col("Bin_Variance")
            .sqrt() // Wendet die Wurzel auf jedes Element an
            .alias("Bin_StdDev")
    )
    .collect()?;

    let pattern = "../di_pressure_profile_at_time_*.csv";

    let mut lfs = Vec::new();
    // 1. Jede Datei einzeln scannen und Spalten umbenennen
    for (i, entry) in glob(pattern).expect("Failed to read glob pattern").enumerate() {
        let path = entry.expect("Invalid path");
        let file_name = path.file_stem().unwrap().to_string_lossy();

        let lf = LazyCsvReader::new(path)
            .with_has_header(true)
            .finish()?
            .select([
                col("Bin"),
                // "Average" wird zu "Average_Datei1", "Average_Datei2", etc.
                col("Average").alias(&format!("Average_{}", i)),
            ]);

        lfs.push(lf);
    }
    // 2. Alle Dateien über die "Bin"-Spalte horizontal zusammenführen
    let mut combined_lf = lfs.remove(0);
    for next_lf in lfs {
        combined_lf = combined_lf.left_join(next_lf, col("Bin"), col("Bin"));
    }

    let n_files = 10; // Beispielhaft für deine Dateien
    let avg_cols: Vec<Expr> = (0..n_files)
        .map(|i| col(&format!("Average_{}", i)))
        .collect();
    // 2. Mittelwert und Standardabweichung horizontal berechnen
    let df_di = combined_lf
        .with_columns([
            avg_cols.iter().cloned()
                .fold(lit(0.0), |acc, x| acc + x)
                .apply(move |s| Ok(Some(s / (n_files) as f64)), GetOutput::from_type(DataType::Float64))
                .alias("Bin_Mean")
        ])
        .with_columns([
            // Horizontale Standardabweichung (Manuelle Formel: sqrt(mean(x^2) - mean(x)^2))
            // Wir berechnen die Summe der quadrierten Abweichungen
            avg_cols.iter().cloned()
                .map(|c| (c - col("Bin_Mean")).pow(2))
                .fold(lit(0.0), |acc, x| acc + x)
                .apply(move |s| Ok(Some(s / ((n_files - 1) as f64))), GetOutput::from_type(DataType::Float64))
                .alias("Bin_Variance")
        ])
        .collect()?;
    let df_di = df_di
    .lazy()
    .with_column(
        col("Bin_Variance")
            .sqrt() // Wendet die Wurzel auf jedes Element an
            .alias("Bin_StdDev")
    )
    .collect()?;

    let pattern = "../se_pressure_profile_at_time_*.csv";

    let mut lfs = Vec::new();
    // 1. Jede Datei einzeln scannen und Spalten umbenennen
    for (i, entry) in glob(pattern).expect("Failed to read glob pattern").enumerate() {
        let path = entry.expect("Invalid path");
        let file_name = path.file_stem().unwrap().to_string_lossy();

        let lf = LazyCsvReader::new(path)
            .with_has_header(true)
            .finish()?
            .select([
                col("Bin"),
                // "Average" wird zu "Average_Datei1", "Average_Datei2", etc.
                col("Average").alias(&format!("Average_{}", i)),
            ]);

        lfs.push(lf);
    }
    // 2. Alle Dateien über die "Bin"-Spalte horizontal zusammenführen
    let mut combined_lf = lfs.remove(0);
    for next_lf in lfs {
        combined_lf = combined_lf.left_join(next_lf, col("Bin"), col("Bin"));
    }

    let n_files = 10; // Beispielhaft für deine Dateien
    let avg_cols: Vec<Expr> = (0..n_files)
        .map(|i| col(&format!("Average_{}", i)))
        .collect();
    // 2. Mittelwert und Standardabweichung horizontal berechnen
    let df_se = combined_lf
        .with_columns([
            avg_cols.iter().cloned()
                .fold(lit(0.0), |acc, x| acc + x)
                .apply(move |s| Ok(Some(s / (n_files) as f64)), GetOutput::from_type(DataType::Float64))
                .alias("Bin_Mean")
        ])
        .with_columns([
            // Horizontale Standardabweichung (Manuelle Formel: sqrt(mean(x^2) - mean(x)^2))
            // Wir berechnen die Summe der quadrierten Abweichungen
            avg_cols.iter().cloned()
                .map(|c| (c - col("Bin_Mean")).pow(2))
                .fold(lit(0.0), |acc, x| acc + x)
                .apply(move |s| Ok(Some(s / ((n_files - 1) as f64))), GetOutput::from_type(DataType::Float64))
                .alias("Bin_Variance")
        ])
        .collect()?;
    let df_se = df_se
    .lazy()
    .with_column(
        col("Bin_Variance")
            .sqrt() // Wendet die Wurzel auf jedes Element an
            .alias("Bin_StdDev")
    )
    .collect()?;

    println!("Kombinierter DataFrame:\n{:?}", df_di_vd);
    println!("Kombinierter DataFrame:\n{:?}", df_di);
    println!("Kombinierter DataFrame:\n{:?}", df_se);

    // 2. Daten extrahieren
    // Wir nutzen f64 für die Berechnung der Achsen-Limits
    let bins_di_vd: Vec<i64> = df_di_vd.column("Bin")?.i64()?.into_no_null_iter().collect();
    let averages_di_vd: Vec<f64> = df_di_vd.column("Bin_Mean")?.f64()?.into_no_null_iter().map(|val| val / 1000.0).collect();
    let deviation_di_vd: Vec<f64> = df_di_vd.column("Bin_StdDev")?.f64()?.into_no_null_iter().map(|val| val / 1000.0).collect();

    let bins_di: Vec<i64> = df_di.column("Bin")?.i64()?.into_no_null_iter().collect();
    let averages_di: Vec<f64> = df_di.column("Bin_Mean")?.f64()?.into_no_null_iter().map(|val| val / 1000.0).collect();
    let deviation_di: Vec<f64> = df_di.column("Bin_StdDev")?.f64()?.into_no_null_iter().map(|val| val / 1000.0).collect();

    let bins_se: Vec<i64> = df_se.column("Bin")?.i64()?.into_no_null_iter().collect();
    let averages_se: Vec<f64> = df_se.column("Bin_Mean")?.f64()?.into_no_null_iter().map(|val| val / 1000.0).collect();
    let deviation_se: Vec<f64> = df_se.column("Bin_StdDev")?.f64()?.into_no_null_iter().map(|val| val / 1000.0).collect();

    // Dynamische Limits für die Achsen bestimmen
    let x_max = *bins_di.iter().max().unwrap_or(&10) as f64;
    let y_max = averages_di.iter().copied().fold(f64::NAN, f64::max) * 1.1; // 10% Puffer oben

    let x_max1 = (*bins_se.iter().max().unwrap_or(&10) as f64).max(x_max);
    let y_max1 = (averages_se.iter().copied().fold(f64::NAN, f64::max) * 1.1).max(y_max); // 10% Puffer oben

    let x_max2 = (*bins_di_vd.iter().max().unwrap_or(&10) as f64).max(x_max);
    let y_max2 = (averages_di_vd.iter().copied().fold(f64::NAN, f64::max) * 1.1).max(y_max); // 10% Puffer oben

    // 3. Plotters Setup
    let root = BitMapBackend::new("pressure_plot1.png", (1600, 1200)).into_drawing_area();
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
        .y_desc("Pressure p [Pa] x 10^3")
        .axis_desc_style(("sans-serif", 40))
        .draw()?;

    let mut points: Vec<(f64, f64)> = bins_di.clone().into_iter().zip(averages_di.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: RED.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Pressure for IISPH with DI source term")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: RED.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    let mut points: Vec<(f64, f64)> = bins_se.clone().into_iter().zip(averages_se.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: BLUE.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Pressure for SESPH")
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
    let root = BitMapBackend::new("pressure_plot2.png", (1600, 1200)).into_drawing_area();
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
        .y_desc("Pressure p [Pa] x 10^3")
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
    .label("Pressure for IISPH with DI source term")
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
    .label("Pressure for IISPH with DI+VD source term")
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

    // Dynamische Limits für die Achsen bestimmen
    let x_max = *bins_di.iter().max().unwrap_or(&10) as f64;
    let y_max = deviation_di.iter().copied().fold(f64::NAN, f64::max) * 1.1; // 10% Puffer oben

    let x_max1 = (*bins_se.iter().max().unwrap_or(&10) as f64).max(x_max);
    let y_max1 = (deviation_se.iter().copied().fold(f64::NAN, f64::max) * 1.1).max(y_max); // 10% Puffer oben

    let x_max2 = (*bins_di_vd.iter().max().unwrap_or(&10) as f64).max(x_max);
    let y_max2 = (deviation_di_vd.iter().copied().fold(f64::NAN, f64::max) * 1.1).max(y_max); // 10% Puffer oben

    // 3. Plotters Setup
    let root = BitMapBackend::new("pressure_dev_plot1.png", (1600, 1200)).into_drawing_area();
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
        .y_desc("Pressure p [Pa] x 10^3")
        .axis_desc_style(("sans-serif", 40))
        .draw()?;

    let mut points: Vec<(f64, f64)> = bins_di.clone().into_iter().zip(deviation_di.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: RED.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Std. deviation of pressure for IISPH with DI source term")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: RED.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    let mut points: Vec<(f64, f64)> = bins_se.clone().into_iter().zip(deviation_se.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: BLUE.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Std. deviation of pressure for SESPH")
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
    let root = BitMapBackend::new("pressure_dev_plot2.png", (1600, 1200)).into_drawing_area();
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
        .y_desc("Pressure p [Pa] x 10^3")
        .axis_desc_style(("sans-serif", 40))
        .draw()?;

    let points: Vec<(f64, f64)> = bins_di.clone().into_iter().zip(deviation_di.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: RED.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Std. deviation of pressure for IISPH with DI source term")
    // .legend(move |(x, y)| Cross::new((x, y), 5, color.filled()));
    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
        color: RED.to_rgba(),
        filled:false,
        stroke_width: 2,
    }));

    let points: Vec<(f64, f64)> = bins_di_vd.clone().into_iter().zip(deviation_di_vd.clone().into_iter()).map(|(b, avg)| {
            (b as f64, avg)
        }).collect();
    chart.draw_series(
        LineSeries::new(points, ShapeStyle {
            color: BLUE.to_rgba(),
            filled:false,
            stroke_width: 2,
        }),
    )?
    .label("Std. deviation of pressure for IISPH with DI+VD source term")
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