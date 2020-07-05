use crate::utils::MeasurementType;
use std::path::PathBuf;
use std::vec::Vec;
use thiserror::Error;

/// A stream of measurements that can be used for running SLAM
pub trait Stream {
    fn measurement_type(&self) -> MeasurementType;

    /// Initialisation actions for the stream at hand
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

/// A stream of finite measurements
pub trait FiniteStream: Stream {
    /// How many measurements does this stream contain
    fn len(&self) -> usize;
    /// Compute and return the frequency of the measurements
    /// In case that is not possible return an error explaining why
    ///
    /// # Errors
    ///
    /// Will return [`DatasetDriverError::UnsteadyFrequency`] in case the frequency is not steady
    fn freq_hint(&self) -> Result<f64, DatasetDriverError>;
}

/// A trait for datasets used for running SLAM
/// The dataset comprises of Stream(s) each of which can bring their own data
///
/// Implementations of this trait are responsible for keeping track of the registered
/// callback functions and serve them whenever new data is available.
///
/// A standard workflow using a dataset should look like this
///
///```text
///           +---+ allow callbacks               start streaming data
///           |     ([`State::Initialised`])      callback registrations forbidden
///           |                                   ([`State::Running`])
///           |                                   +
///           |                                   |
/// +------+  v    +----------+      +-------+    v
/// |init()+--+--->+lockdown()+---+->+start()+----+--->
/// +------+       +----------+   ^  +-------+
///                               |
///                               |
///                               +----+  callback registrations now allowed
///                                       ([`State::Locked`])
///```
pub trait DatasetDriver {
    /// return the path to the root directory of the dataset
    fn root_dir(&self) -> Option<&PathBuf>;

    fn num_streams(&self) -> usize {
        self.all_streams().len()
    }

    fn num_enabled_streams(&self) -> usize {
        self.enabled_streams().len()
    }

    /// Return shared references of the underlying streams. Knowing the available streams is
    /// required for registering callbacks to their data
    fn all_streams(&self) -> Vec<&dyn Stream>;

    /// Return shared references of the underlying *enabled* streams.
    fn enabled_streams(&self) -> Vec<&dyn Stream> {
        self.all_streams()
            .into_iter()
            .filter(|&stream| self.is_enabled(stream))
            .collect()
    }

    /// Return shared references of the underlying *disabled* streams.
    fn disabled_streams(&self) -> Vec<&dyn Stream> {
        self.all_streams()
            .into_iter()
            .filter(|&stream| !self.is_enabled(stream))
            .collect()
    }

    /// Enable the said stream
    fn enable_stream(&mut self, stream: &dyn Stream);

    /// Disable the said stream
    fn disable_stream(&mut self, stream: &dyn Stream);

    /// Is the given stream enabled?
    fn is_enabled(&self, stream: &dyn Stream) -> bool;

    /// Return the number of registered callbacks
    fn num_callbacks() -> usize;

    /// Register a function that is called when the next data is available.
    /// This function will receive a shared ref to the underlying data and should call `clone` to if
    /// they want to modify it.
    ///
    /// # Errors
    ///
    /// Returns the appropriate error in case the registration was unsuccessful (for example when
    /// the measurement type is wrong)
    fn register_callback<T>(
        s: &dyn Stream,
        f: fn(T) -> Result<(), DatasetDriverError>,
    ) -> Result<(), DatasetDriverError>;

    /// Do an initial pass on the dataset
    /// This could entail:
    /// - Validating the dataset contents
    /// - Parsing of the metadata
    /// - Reading the intial measurements etc.
    fn init() -> Result<(), DatasetDriverError>;

    /// Disallow additional registrations from this point on.
    /// Will fail if there are no registered callbacks
    fn lockdown() -> Result<(), DatasetDriverError>;

    /// Return the state of the current dataset driver
    fn state() -> DatasetDriverState;

    /// Start reading the data and serving the registered callbacks.
    fn start() -> Result<(), DatasetDriverError>;
}

/// State that the potential Dataset driver may be at each time
#[derive(Debug)]
pub enum DatasetDriverState {
    Uninitialised,
    Initialised,
    Locked,
    Running,
}

/// Errors associated with dataset operations
#[derive(Error, Debug)]
pub enum DatasetDriverError {
    #[error("Frequency of the given stream is not steady")]
    UnsteadyFrequency,
    #[error("Dataset is not initialised yet")]
    DatasetNotInitialised,
    #[error("Dataset initialisation failed - Reason: {0}")]
    InitDatasetError(String),
    #[error("Stream is disabled")]
    StreamDisabled,
    #[error("End of data in stream")]
    EndOfStream,
    #[error("Stream doesn't contain any measurements")]
    StreamEmpty,
    #[error("Unknown dataset-related error")]
    Unknown,
}
