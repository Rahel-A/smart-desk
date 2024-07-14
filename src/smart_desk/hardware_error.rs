use std::fmt::{Display, Formatter};

#[derive(Debug)]

pub enum HardwareError {
    NoDistance,
    DistanceOutOfRange,
    NotConfigured,
}

impl Display for HardwareError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoDistance => write!(f, "Distance measurement failed"),
            Self::DistanceOutOfRange => write!(f, "Object out of range"),
            Self::NotConfigured => write!(f, "Hardware not configured"),
        }
    }
}

