
//! Measure with rust_fluid_solver
//!
//!
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use itertools::Itertools;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::FmtSubscriber;
use indicatif::{MultiProgress, ProgressBar};
use indicatif::style::ProgressStyle;



/// Simple fluid solver written in rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Store measurements to .csv file
    // #[arg(short, long,)]
    measurement_file: String,
    /// Exit when finished
    #[arg(short, long,)]
    exit: bool,
    /// Log severity level (Options: TRACE, DEBUG, INFO, WARN, ERROR, OFF)
    #[arg(short, long, default_value_t=String::from("INFO"))]
    log: String,
}

fn create_folder_or_error(path: &str) -> std::io::Result<()> {
    let folder = Path::new(path);

    if folder.exists() {
        // Folder already exists → return an error
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Folder '{}' already exists", path),
        ))
    } else {
        // Folder does not exist → create it
        std::fs::create_dir_all(folder)?;
        Ok(())
    }
}

fn add_file_name_to_folder(folder: &str, file: &str) -> String {
    let folder = Path::new(folder);
    let file_path = folder.join(file);
    file_path.to_string_lossy().into_owned()
}

fn add_suffix(original: &str, suffix: &str) -> std::path::PathBuf {
    let path = Path::new(original);

    // Split filename and extension
    let stem = path.file_stem().unwrap().to_string_lossy();
    let ext  = path.extension().unwrap_or_default().to_string_lossy();

    // Build new filename: name.suffix.ext
    let new_filename = format!("{}_{}.{}", stem, suffix, ext);

    // Return new path in the same directory
    path.with_file_name(new_filename)
}

#[derive(Debug, Deserialize)]
struct MeasurementConfig {
    general: General,
    operation: Operation,
    parameters: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize)]
struct General {
    config_file: String,
    state_file: String,
    measurement_destination_file_path: String,
    start_time: f64,
    finish_time: f64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "parameters")]
enum Operation {
    Zip,
    Cartesian,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub parameters: HashMap<String, toml::Value>,
    pub light: HashMap<String, toml::Value>,
    pub scene: HashMap<String, toml::Value>,
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

    // Read file into a string
    let content_string = match std::fs::read_to_string(args.measurement_file) {
        Ok(content) => content,
        Err(e) => panic!("Could not read measurement file: {}", e)
    };
    // Parse
    let measurement_config: MeasurementConfig = toml::from_str(&content_string)?;

    // Handling file paths
    let extension = "temp".to_string();
    let temp_file_path = add_suffix(&measurement_config.general.config_file, &extension);
    if temp_file_path.as_path().exists() {
        panic!("Parameter variation config file already exists!");
    }

    // Read file into a string
    let content_string = std::fs::read_to_string(measurement_config.general.config_file)?;
    // Parse into a TOML value
    let mut config_content: Setup = toml::from_str(&content_string)?;

    match create_folder_or_error(&measurement_config.general.measurement_destination_file_path) {
        Ok(_) => println!("Created folder: {}", &measurement_config.general.measurement_destination_file_path),
        Err(e) => panic!("Error: {}", e),
    }

    let parameter_combinations: Vec<HashMap<String, toml::Value>> = match measurement_config.operation {
        Operation::Zip => {
            let min_len = measurement_config.parameters.values().map(|v| {
                v.as_array().expect("Parameters must be defined in an array").len()
            }).min().unwrap();
            let mut parameter_combinations = Vec::with_capacity(min_len);
            for i in 0..min_len {
                let mut combination = HashMap::new();
                for (key, array) in &measurement_config.parameters {
                    combination.insert(key.clone(), array[i].clone());
                }
                parameter_combinations.push(combination);
            }
            parameter_combinations
        },
        Operation::Cartesian => {
            let keys: Vec<String> = measurement_config.parameters.keys().cloned().collect();
            // Get a Vec of iterators over each value list
            let list_iters: Vec<Vec<toml::Value>> = keys
                .iter()
                .map(|k| measurement_config.parameters[k].as_array().expect("Parameters must be defined in an array").clone())
                .collect();
            let mut parameter_combinations = Vec::new();
            for value_combi in list_iters.into_iter().multi_cartesian_product() {
                let mut combination = HashMap::new();
                for (key, val) in keys.iter().zip(value_combi.into_iter()) {
                    combination.insert(key.clone(), val.clone());
                }
                parameter_combinations.push(combination);
            }
            parameter_combinations
        },
    };

    println!("Close rusty fluid solver to proceed with next measurement, when a measurement is finished.");

    // init progress bar (for optimum performance remove bar)
    let multi_p_bar = MultiProgress::new();
    let bar_style = ProgressStyle::with_template(
            "[{elapsed_precise}]{wide_bar:.cyan/blue} {pos}/{len} measurements [{eta_precise}]\n{msg}").unwrap();
    let bar = multi_p_bar.add(ProgressBar::new(parameter_combinations.len() as u64));
    bar.set_style(bar_style.clone());

    for combi in parameter_combinations {
        println!("Measuring parameter combination:");
        // Modify values
        for (k, v) in combi.clone() {
            if config_content.parameters.insert(k.clone(), v.clone()).is_none() {
                panic!("Parameter in measurement.parameters does not match parameter in config file.
                Either parameter in measurement.parameters is wrong or parameter in config file is missing.");
            }
            println!("{}: {}", k, v)
        }

        // Convert back to TOML string
        let updated_content_hash_map = toml::to_string_pretty(&config_content)?;

        // Save back to file
        std::fs::write(temp_file_path.clone(), updated_content_hash_map)?;

        let file_name: String = combi
            .iter()
            .map(|(k, v)| format!("{}_{}", k, v))
            .collect::<Vec<_>>()
            .join("_");

        let file_name = format!("{}.csv", file_name);

        // call fluid solver
        let mut child = Command::new("../rusty_fluid_solver/target/release/rusty_fluid_solver")        // executable
            .arg(temp_file_path.clone())
            .arg("-s")
            .arg(measurement_config.general.state_file.clone())
            .arg("-m")
            .arg(add_file_name_to_folder(&measurement_config.general.measurement_destination_file_path, &file_name))
            .arg("--start_time")
            .arg(measurement_config.general.start_time.to_string())
            .arg("-f")
            .arg(measurement_config.general.finish_time.to_string())
            // .arg("-r")
            // .arg("-e")
            .arg("-l")
            .arg("INFO")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        let bar_clone = bar.clone();

        let handle = std::thread::spawn(move || {
            for line in reader.lines() {
                let line = line.unwrap();
                bar_clone.println(line); // prints above the progress bar
            }
        });

        handle.join().unwrap();
        let _status = child.wait().expect("Failed to wait on child");

        bar.inc(1);
    }
    bar.finish_with_message("All measurements done!");

    if temp_file_path.as_path().exists() && temp_file_path.as_path().is_file() {
        std::fs::remove_file(temp_file_path)?;
    }

    Ok(())
}
