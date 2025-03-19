use serde::{Deserialize, Serialize};

use crate::components::CommuneProgressClass;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Progress {
    #[serde(rename = "type")]
    pub(crate) progress_type: String,
    pub(crate) features: Vec<CommuneData>,
    pub(crate) date: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommuneData {
    #[serde(rename = "type")]
    pub(crate) feature_type: String,
    pub(crate) geometry: Geometry,
    pub(crate) properties: CommuneDynamicData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub(crate) geometry_type: String,
    pub(crate) coordinates: Vec<f64>,
}

/// The data returned by the live API, i.e. the stuff we query upon page load
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommuneDynamicData {
    pub(crate) name: String,
    pub(crate) population: i64,
    pub(crate) contributions: i64,
    pub(crate) per_cent: f64,
    pub(crate) insee: String,
}

/// This is the data we can prepare and bulk-load for all communes since it
/// won't change over the course of the baromètre
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommuneStaticData {
    /// INSEE code; roughly similar to code postale but NOT the same
    pub(crate) insee: String,
    /// name. we need this since the current live api will only return data for
    /// communes with at least one response
    pub(crate) name: String,
    /// true IFF population is >= 5000 according to INSEE
    pub(crate) is_big: bool,
    pub(crate) contributions_2021: usize,
}

/// The data that we actually want to display.
/// This is a mix of historic data (previous baromètre),
/// static data (INSEE) and dynamic data (current baromètre)
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommuneDisplayProps {
    pub(crate) name: String,
    pub(crate) is_big: bool,
    pub(crate) contributions: i64,
    pub(crate) contributions_2021: usize,
    pub(crate) insee: String,
}

// impl Into<CommunePropertiesSlim> for CommuneDynamicData {
//     fn into(self) -> CommunePropertiesSlim {
//         CommunePropertiesSlim {
//             name: self.name,
//             is_big: self.population >= 5000,
//             contributions: self.contributions,
//             insee: self.insee,
//         }
//     }
// }

pub fn progress_class_to_color(progress_class: CommuneProgressClass) -> &'static str {
    match progress_class {
        CommuneProgressClass::Qualified => "green",
        CommuneProgressClass::Close => "orange",
        CommuneProgressClass::NonZero => "red",
        // not important since zero progress will never actually draw any pixels, but we need to return *something*
        CommuneProgressClass::Zero => "slate",
    }
}

impl CommuneDisplayProps {
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
        if self.is_big {
            50
        } else {
            30
        }
    }
}
