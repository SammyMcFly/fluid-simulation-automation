# Rusty Measurement Runner

An automation tool for executing parametric measurement series with `rusty_fluid_solver`.

## Overview

`rusty_measurement_runner` automates the execution of `rusty_fluid_solver` across a predefined set of simulation parameter combinations. It reads a measurement configuration file, either generates all parameter permutations or zips all parameter lists, and runs the solver for each combination — collecting `.csv` measurement outputs into a destination folder.

This is useful for:

- Parameter sweeps (e.g. varying viscosity, particle spacing, time step size)
- Generating reproducible measurement series for analysis
- Batch simulation runs without manual intervention

## How It Works

1. Parse measurement config (.csv / .toml)
2. Zip parameters or compute cartesian product of parameter combinations
3. For each combination:
-  Patch the scene config file with current parameters
- Write a temporary config file
- Spawn rusty_fluid_solver as a subprocess
- Stream solver output to terminal
- Collect .csv measurement results
4. Clean up temporary files

### Output Structure

```
measurement_destination_folder/
├── measurement_config_copy.toml      # Copy of the measurement config
├── scene_config_copy.toml            # Copy of the original scene config
├── paramA_val1_paramB_val1.csv       # Measurement for combination 1
├── paramA_val1_paramB_val2.csv       # Measurement for combination 2
├── paramA_val2_paramB_val1.csv       # Measurement for combination 3
└── ...
```

Output filenames are automatically generated from the parameter names and values (sorted alphabetically), e.g. `viscosity_0.01_particle_spacing_0.02.csv`.

## Usage

```bash
cargo run --release -p rusty_measurement_runner -- [OPTIONS] <MEASUREMENT_FILE>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `measurement_file` | Path to the measurement configuration file |

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--exit` | `-e` | Automatically exit each solver run when the measurement is finished and proceed to the next |
| `--log <LEVEL>` | `-l` | Log severity: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, `OFF` (default: `INFO`) |

### Examples

```bash
# Run a full measurement series with auto-exit
cargo run --release -p rusty_measurement_runner -- measurements.toml --exit

# Run interactively (manually close each solver window to proceed)
cargo run --release -p rusty_measurement_runner -- measurements.toml

# With debug logging
cargo run --release -p rusty_measurement_runner -- measurements.toml --exit --log DEBUG
```

## Measurement Configuration

The measurement configuration file defines:

- **General settings** — paths, time range, executable location
- **Parameter series** — which parameters to vary and their values

### Configuration Fields

| Section | Field | Description |
|---------|-------|-------------|
| `general` | `config_file` | Path to the base scene configuration `.toml` file |
| `general` | `executable` | Path to the `rusty_fluid_solver` binary |
| `general` | `measurement_destination_file_path` | Output folder for results |
| `general` | `start_time` | Simulation time at which measurement begins |
| `general` | `finish_time` | Simulation time at which measurement ends |
| `general` | `state_file` | (Optional) Path to a saved state file to resume from |
| `parameters` | `<param_name>` | List of values to sweep for each parameter, `<param_name>` can be any field from `parameters` in the fluid simulations configuration |

### Example Configuration

```toml
[general]
config_file = "scene_config.toml"
executable = "./target/release/rusty_fluid_solver"
measurement_destination_file_path = "output/measurements/"
start_time = 0.0
finish_time = 5.0
state_file = ""

[operation]
type = "Cartesian" # Zip, Cartesian

[parameters]
viscosity = [0.01, 0.05, 0.1]
time_increment = [0.005, 0.01]
```

This configuration produces 3 × 2 = **6 simulation runs**, one for each combination of `viscosity` and `time_increment`.

## Behavior

- **Parameter validation** — The runner verifies that each parameter listed in the measurement config exists in the scene config file. If a parameter is not found, the program panics with a descriptive error.
- **File preservation** — Both the measurement config and the original scene config are copied to the destination folder for reproducibility.
- **Temporary files** — A temporary patched config file is created during execution and cleaned up after all measurements complete.
- **Progress tracking** — A progress bar (`indicatif`) shows overall completion status and estimated time remaining.
- **Solver output** — stdout from each `rusty_fluid_solver` invocation is streamed above the progress bar.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `toml` | Scene config parsing and patching |
| `tracing` / `tracing-subscriber` | Structured logging |
| `indicatif` | Progress bar display |
| `std::process::Command` | Subprocess management for solver invocation |

## Relationship to rusty_fluid_solver

This tool is a **companion program** — it does not contain simulation logic itself. It orchestrates multiple runs of `rusty_fluid_solver` by:

1. Modifying the scene config's `[parameters]` section for each combination
2. Passing appropriate CLI flags (`-m`, `-s`, `-f`, `-r`, `-e`, `--state`)
3. Collecting the resulting `.csv` measurement files in one location