use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{WakewordLoad, WakewordRef};

/// Deprecated wakeword representation from v2
#[derive(Serialize, Deserialize)]
pub struct WakewordV2 {
    pub name: String,
    pub avg_features: Option<Vec<Vec<f32>>>,
    pub samples_features: HashMap<String, Vec<Vec<f32>>>,
    pub threshold: Option<f32>,
    pub avg_threshold: Option<f32>,
    pub rms_level: f32,
    pub enabled: bool,
}
impl WakewordLoad for WakewordV2 {}
impl TryFrom<WakewordV2> for WakewordRef {
    type Error = String;

    fn try_from(val: WakewordV2) -> Result<Self, Self::Error> {
        let mfcc_size = val
            .samples_features
            .values()
            .next()
            .ok_or_else(|| "Can not convert an empty wakeword".to_string())?[0]
            .len() as u16;
        Ok(WakewordRef {
            name: val.name,
            mfcc_size,
            threshold: val.threshold,
            avg_threshold: val.avg_threshold,
            avg_features: val.avg_features,
            samples_features: val.samples_features,
            rms_level: val.rms_level,
        })
    }
}
