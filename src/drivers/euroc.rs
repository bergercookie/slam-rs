/// Read data from a dataset in the `EuRoC` format
/// For more information on the latter see the following:
///
/// - [`EuRoC` datasets download page](https://projects.asl.ethz.ch/datasets/doku.php?id=kmavvisualinertialdatasets)
/// - [Paper](https://www.researchgate.net/publication/280596082_Vision-based_localization_mapping_and_control_for_autonomous_MAV_EuRoC_challenge_results)
use crate::drivers::traits::{
    DatasetDriver, DatasetDriverError, DatasetDriverState, FiniteStream, Stream,
};
use crate::utils::{Measurement, MeasurementData, MeasurementType};

use image::{open, GrayImage};
use std::path::PathBuf;
use std::time::Duration;

use csv::Reader;
use csv::Result as CsvResult;
use log::{info, warn};
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
use mocktopus::macros::*;

// -------------------------------------------------------------------------------------------------
// EurocStreamGray
// -------------------------------------------------------------------------------------------------

/// A stream of data that come from a euroc dataset
/// Each one of the various measurement types (Gray, Odometry, GPS) will have its own struct.
///
/// TODO: Extend this either with generics or by wrapping it in a macro
/// TODO: Add a customisation point - provide params such as images suffix to use
#[derive(Debug, Default)]
pub struct EurocStreamGray {
    /// Path to the root directory containing the measurements *of the current stream*
    /// Notice that this differs from the root directory of the overall dataset
    ///
    /// <...>/mav0           <--- root dir of dataset
    /// <...>/mav0/cam0/     <--- root dir of stream
    root_dir: PathBuf,
    /// Frequency of the measurements in the stream
    freq: Option<f64>,
    /// Names of the images for the given stream (not the full path to the images, just the basename)
    img_paths: Vec<PathBuf>,
    /// Points to the next camera measurements that is to be read
    stream_cursor: usize,
}

#[cfg_attr(test, mockable)]
impl EurocStreamGray {
    pub fn new() -> Self {
        EurocStreamGray {
            root_dir: PathBuf::new(),
            freq: None,
            img_paths: Vec::new(),
            stream_cursor: 0,
        }
    }

    /// Set the root directory
    pub fn root_dir_mut(&mut self, root_dir: PathBuf) -> &Self {
        self.root_dir = root_dir;
        self
    }
    pub fn root_dir(mut self, root_dir: PathBuf) -> Self {
        self.root_dir = root_dir;
        self
    }

    fn image_exists(&self, img_path: &PathBuf) -> bool {
        img_path.exists()
    }
}

impl Iterator for EurocStreamGray {
    type Item = MeasurementData;
    /// Get the next image in the stream
    fn next(&mut self) -> Option<Self::Item> {
        if self.stream_cursor == self.img_paths.len() {
            None
        } else {
            let path = &self.img_paths[self.stream_cursor];
            self.stream_cursor += 1;

            let img = open(path).unwrap().into_luma();
            Some(MeasurementData::Grayscale(img))
        }
    }
}

impl Stream for EurocStreamGray {
    fn measurement_type(&self) -> MeasurementType {
        MeasurementType::Grayscale
    }

    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // get a list of all the timestamps and image strings

        // initialise reader
        let f = File::open(self.root_dir.join("data.csv"))?;
        let mut conts = vec![];
        f.read_to_end(&mut conts);
        let rdr = Reader::from_reader(&conts);

        let csv_iter = rdr.into_records();
        // TODO - Does this include the header?

        let mut img_stamps: Vec<Duration> = vec![];

        // discard images that are not actually found in the dataset - inform about it
        for result in csv_iter {
            let record = result?;
            let nsecs = Duration::from_nanos(record[0].parse::<u64>()?);

            let img_path = self.root_dir.join(&record[1]);
            if !self.image_exists(&img_path) {
                warn!("Image path [{}] is invalid", img_path.display());
                continue;
            }

            img_stamps.push(nsecs);
            self.img_paths.push(img_path);
        }

        if img_stamps.is_empty() {
            return Err(Box::new(DatasetDriverError::StreamEmpty));
        }

        // cache frequency -------------------------------------------------------------------------
        // all frequencies
        let mut freqs = Vec::<f64>::with_capacity(img_stamps.len() - 1);
        for i in 1..img_stamps.len() {
            freqs[i - 1] = 1.0 / (img_stamps[i] - img_stamps[i - 1]).as_secs_f64()
        }

        // compute mean
        let mean: f64 = freqs.iter().sum::<f64>() / freqs.len() as f64;

        // compute stddev
        let variance: f64 =
            freqs.iter().map(|freq| (freq - mean).powi(2)).sum::<f64>() / freqs.len() as f64;
        let stddev = variance.sqrt();
        println!("stddev: {:#?}", stddev);

        // if most data (90%) are in the [-0.3sigma, +0.3sigma] range then mean == freq_hint
        if freqs
            .iter()
            .filter(|&&freq| freq > 3.0f64.mul_add(stddev, mean) || freq < mean - 3.0 * stddev)
            .count()
            < (0.1 * freqs.len() as f64) as usize
        {
            self.freq = Some(mean);
        }

        Ok(())
    }
}

impl FiniteStream for EurocStreamGray {
    fn len(&self) -> usize {
        self.img_paths.len()
    }

    fn freq_hint(&self) -> Result<f64, DatasetDriverError> {
        match self.freq {
            Some(f) => Ok(f),
            None => Err(DatasetDriverError::UnsteadyFrequency),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// EurocDriver
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct EurocDriver {
    /// Path to the root directory of this dataset
    root_dir: PathBuf,
    streams_gray: Vec<EurocStreamGray>,
    // TODO - Add more streams for each type of data taht we support in the Euroc dataset
}

/// TODO: Finish DatasetDriver
/// TODO: Create a toy camera_viewer app to display everything
impl DatasetDriver for EurocDriver {
    fn root_dir(&self) -> Option<&PathBuf> {
        Some(&self.root_dir)
    }

    fn all_streams(&self) -> Vec<&dyn Stream> {
        let mut vec = Vec::<&dyn Stream>::with_capacity(self.streams_gray.len());
        for s in &self.streams_gray {
            vec.push(s);
        }

        vec
    }

    fn enable_stream(&mut self, stream: &dyn Stream) {
        todo!()
    }

    fn disable_stream(&mut self, stream: &dyn Stream) {
        todo!()
    }

    fn is_enabled(&self, stream: &dyn Stream) -> bool {
        todo!()
    }

    fn num_callbacks() -> usize {
        todo!()
    }

    fn register_callback<T>(
        s: &dyn Stream,
        f: fn(T) -> Result<(), DatasetDriverError>,
    ) -> Result<(), DatasetDriverError> {
        todo!()
    }

    fn init() -> Result<(), DatasetDriverError> {
        todo!()
    }

    fn lockdown() -> Result<(), DatasetDriverError> {
        todo!()
    }

    fn state() -> DatasetDriverState {
        todo!()
    }

    fn start() -> Result<(), DatasetDriverError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mocktopus::mocking::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn euroc_stream_invalid_dir_test() {
        let mut stream = EurocStreamGray::new().root_dir("some-directory".into());
        let _ = stream.init();
        match Error::last_os_error().raw_os_error() {
            Some(2) => {}
            Some(_) | None => panic!("Should have failed"),
        }
    }

    #[test]
    fn euroc_stream_normal_test() {
        EurocStreamGray::image_exists.mock_safe(|x, y| MockResult::Return(true));
        EurocStreamGray::get_csv_reader.mock_safe(|x| MockResult::Return(true));

        let mut stream = EurocStreamGray::new();
        let mut fake_csv_reader = todo!(); // TODO
        let stream_images: Vec<MeasurementData> = vec![]; // TODO
        stream.get_csv_reader().returning(|| Ok(fake_csv_reader));
        stream.image_exists().return_const(true);

        stream.init().unwrap();

        for (idx, data) in &stream.iter().enumerate() {
            assert_eq!(stream_images[idx], data)
        }
    }

    // use guerrilla
}
