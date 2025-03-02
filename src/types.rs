use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Progress {
    #[serde(rename = "type")]
    pub(crate) progress_type: String,
    pub(crate) features: Vec<CommuneData>,
    pub(crate) date: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommuneData {
    #[serde(rename = "type")]
    pub(crate) feature_type: String,
    pub(crate) geometry: Geometry,
    pub(crate) properties: CommuneProperties,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub(crate) geometry_type: String,
    pub(crate) coordinates: Vec<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommuneProperties {
    pub(crate) name: String,
    pub(crate) population: i64,
    pub(crate) contributions: i64,
    pub(crate) per_cent: f64,
    pub(crate) insee: String,
}
