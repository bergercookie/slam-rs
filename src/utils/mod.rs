pub mod errors;
pub use self::errors::*;

use image::GrayImage;

/// Types of measurements that we can use to run SLAM with.
#[derive(Debug)]
pub enum MeasurementType {
    Grayscale,
    RGB,      // not implemented
    IMU,      // not implemented
    GPS,      // not implemented
    Odometry, // not implemented
}

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum MeasurementData {
    Grayscale(GrayImage),
    // --- rest not implemented yet
}

/// Implementation for the actual measurements
#[derive(Debug)]
pub struct Measurement {
    measurement_type: MeasurementType,
    data: MeasurementData,
}
