use serde::{Deserialize, Serialize};

use crate::components::CommuneProgressClass;

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

pub fn progress_class_to_color(progress_class: CommuneProgressClass) -> &'static str {
    match progress_class {
        CommuneProgressClass::Qualified => "green",
        CommuneProgressClass::Close => "orange",
        CommuneProgressClass::NonZero => "red",
        // not important since zero progress will never actually draw any pixels, but we need to return *something*
        CommuneProgressClass::Zero => "slate",
    }
}

impl CommunePropertiesSlim {
    pub fn progress(&self) -> f32 {
        (100.0 * self.contributions as f32 / self.target_contributions() as f32).min(100.0)
    }

    pub fn progress_class(&self) -> CommuneProgressClass {
        if self.contributions == 0 {
            return CommuneProgressClass::Zero;
        }
        if self.contributions >= self.target_contributions() as i64 {
            return CommuneProgressClass::Qualified;
        }
        let missing = self.target_contributions() - self.contributions as usize;
        if missing <= 10 {
            CommuneProgressClass::Close
        } else {
            CommuneProgressClass::NonZero
        }
    }

    pub fn target_contributions(&self) -> usize {
        if self.population < 5000 {
            30
        } else {
            50
        }
    }
}
