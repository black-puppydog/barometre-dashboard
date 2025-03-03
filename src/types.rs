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
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommunePropertiesSlim {
    pub(crate) name: String,
    pub(crate) population: i64,
    pub(crate) contributions: i64,
    pub(crate) insee: String,
}

impl Into<CommunePropertiesSlim> for CommuneProperties {
    fn into(self) -> CommunePropertiesSlim {
        CommunePropertiesSlim {
            name: self.name,
            population: self.population,
            contributions: self.contributions,
            insee: self.insee,
        }
    }
}

impl CommunePropertiesSlim {
    pub fn progress(&self) -> f32 {
        (100.0 * self.contributions as f32 / self.target_contributions() as f32).min(100.0)
    }

    pub fn target_contributions(&self) -> usize {
        if self.population < 5000 {
            30
        } else {
            50
        }
    }
}
