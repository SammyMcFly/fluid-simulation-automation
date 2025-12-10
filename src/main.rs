
//! Measure with rust_fluid_solver
//!
//!
use clap::Parser;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use tracing::level_filters::LevelFilter;
use tracing_subscriber::FmtSubscriber;
use indicatif::{MultiProgress, ProgressBar};
use indicatif::style::ProgressStyle;

use config::{MeasurementConfig};

mod config;
mod rusty_fluid_solver;
mod file_operations;


/// Simple fluid solver written in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Store measurements to .csv file
    // #[arg(short, long,)]
    measurement_file: String,
    /// Exit rusty_fluid_solver whenever a measurement is finished and continue with next measurement
    #[arg(short, long,)]
    exit: bool,
    /// Log severity level (Options: TRACE, DEBUG, INFO, WARN, ERROR, OFF)
    #[arg(short, long, default_value_t=String::from("INFO"))]
    log: String,
}

/// Init logging
fn init_logging(args: &Args) {
    let severity_level = match &args.log[..] {
        "TRACE" => LevelFilter::TRACE,
        "DEBUG" => LevelFilter::DEBUG,
        "INFO" => LevelFilter::INFO,
        "WARN" => LevelFilter::WARN,
        "ERROR" => LevelFilter::ERROR,
        _ => LevelFilter::OFF,
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(severity_level)
        .with_writer(std::io::stdout)
        .with_line_number(true)
        // .with_ansi(false)
        // .pretty()
        .finish();
        // .with(debug_log);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // parse args
    let args = Args::parse();

    init_logging(&args);

    // Parse measurement config file
    let measurement_config = MeasurementConfig::parse_from_file(&args.measurement_file)?;
    // combine parameters
    let measurement_series = measurement_config.get_measurement_series();

    // Handling file paths
    let temp_file_path = file_operations::get_temporary_config_file_path(
        &measurement_config.general.config_file,
        &measurement_config.general.measurement_destination_file_path,
    );

    // Read file into a string
    let content_string = std::fs::read_to_string(&measurement_config.general.config_file)?;
    // Parse into a TOML value
    let mut config_content: rusty_fluid_solver::Setup = toml::from_str(&content_string)?;

    match file_operations::create_folder_or_error(&measurement_config.general.measurement_destination_file_path) {
        Ok(_) => println!("Created folder: {}", &measurement_config.general.measurement_destination_file_path),
        Err(e) => panic!("Error: {}", e),
    }

    // copy measurement file and config file to destination folder
    std::fs::copy(
        &args.measurement_file,
        std::path::Path::new(&measurement_config.general.measurement_destination_file_path).join(
            std::path::Path::new(&args.measurement_file).file_name().expect("Could not extract measurement_file file name.")
        ))?;
    std::fs::copy(
        &measurement_config.general.config_file,
        std::path::Path::new(&measurement_config.general.measurement_destination_file_path).join(
            std::path::Path::new(&measurement_config.general.config_file).file_name().expect("Could not extract measurement_file file name.")
        ))?;

    if args.exit {
        println!("Close rusty fluid solver to proceed with next measurement, when a measurement is finished.");
    }

    // init progress bar (for optimum performance remove bar)
    let multi_p_bar = MultiProgress::new();
    let bar_style = ProgressStyle::with_template(
            "[{elapsed_precise}]{wide_bar:.cyan/blue} {pos}/{len} measurements [{eta_precise}]\n{msg}").unwrap();
    let bar = multi_p_bar.add(ProgressBar::new(measurement_series.len() as u64));
    bar.set_style(bar_style.clone());

    for combi in measurement_series {
        bar.println("Measuring parameter combination:");
        // Modify values
        for (k, v) in combi.clone() {
            if config_content.parameters.insert(k.clone(), v.clone()).is_none() {
                panic!("Parameter in measurement.parameters does not match parameter in config file.
                Either parameter in measurement.parameters is wrong or parameter in config file is missing.");
            }
            bar.println(format!("{}: {}", k, v));
        }

        // Convert back to TOML string
        let updated_content_hash_map = toml::to_string_pretty(&config_content)?;

        // Save back to file
        std::fs::write(temp_file_path.clone(), updated_content_hash_map)?;

        let mut combi: Vec<_> = combi.iter().collect();
        combi.sort_by_key(|(k, _)| *k);
        let measurement_file_name: String = combi
            .iter()
            .map(|(k, v)| format!("{}_{}", k, v))
            .collect::<Vec<_>>()
            .join("_");

        let measurement_file_name = format!("{}.csv", measurement_file_name);

        // call fluid solver
        let mut cmd = Command::new(measurement_config.general.executable.clone());
        cmd.arg(temp_file_path.clone())
            .arg("--state")
            .arg(measurement_config.general.state_file.clone())
            .arg("-m")
            .arg(file_operations::add_file_name_to_folder(
                &measurement_config.general.measurement_destination_file_path,
                &measurement_file_name,
            ))
            .arg("-s")
            .arg(measurement_config.general.start_time.to_string())
            .arg("-f")
            .arg(measurement_config.general.finish_time.to_string())
            .arg("-r"); // start resumed
        if args.exit {
            cmd.arg("-e");
        }
        let mut child = cmd.arg("-l")
            .arg("INFO")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        // print solver output above the progress bar
        let bar_clone = bar.clone();
        let handle = std::thread::spawn(move || {
            for line in reader.lines() {
                let line = line.unwrap();
                bar_clone.println(line);
            }
        });

        // finish measurement
        handle.join().unwrap();
        let _status = child.wait().expect("Failed to wait on child");

        // increase progress
        bar.inc(1);
    }
    bar.finish_with_message("All measurements done!");

    // remove temporary file
    if temp_file_path.as_path().exists() && temp_file_path.as_path().is_file() {
        std::fs::remove_file(temp_file_path)?;
    }

    Ok(())
}
