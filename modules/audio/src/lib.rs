use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}
