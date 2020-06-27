extern crate clap;
extern crate slam_rs;

use clap::{App, Arg};
use slam_rs::errors::{SlamError, SlamErrorKind};
use std::error;
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn error::Error>> {
    // --------------------------------------------------------------------------------------------
    // argument parsing
    // --------------------------------------------------------------------------------------------

    let matches = App::new("SLAM Runner")
        .version("0.1.0")
        .author("Nikos Koukis <nickkouk@gmail.com>")
        .arg(
            Arg::with_name("dataset")
                .short('d')
                .long("dataset")
                .takes_value(true)
                .about("Path to the dataset root directory")
                .required(true),
        )
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config-file")
                .takes_value(true)
                .about("Path to the SLAM configuration file"),
        )
        .get_matches();

    // FIXME: Do this as part of a CLI validator struct
    let dataset_path = PathBuf::from(matches.value_of("dataset").unwrap());
    if !dataset_path.exists() {
        return Err(SlamError::new(SlamErrorKind::InvalidCLI(
            "dataset".into(),
            format!("Dataset not found in path [{}]", dataset_path.display()),
        ))
        .into());
    }

    // let driver = EurocDatasetDriver::new(EurocDatasetParams { dataset_path });

    // ---------------------------------------------------------------------------------------------
    // Initialisation
    // ---------------------------------------------------------------------------------------------

    // TODO - What structs to use?

    // read all measurements
    // TODO

    // read configuration
    // TODO

    // GUI Initialisation
    // TODO

    // Initialise Frontend and Backend
    // TODO

    // Build SLAM Object
    // TODO

    // Run the SLAM Loop + Update the GUI
    // TODO Offload the former into a separate thread
    // TODO
    Ok(())
}
