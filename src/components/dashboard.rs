use dioxus::prelude::*;

use crate::components::commune_progress::Commune;
use crate::components::DashboardSummary;
use crate::types::{CommuneData, CommuneProperties, CommunePropertiesSlim};
use std::collections::HashMap;

const PROGRESS_URL: &'static str = "https://www.barometre-velo.fr/stats/progress.geojson";

fn read_insee_codes() -> serde_json::Result<HashMap<String, String>> {
    let insee_codes = include_str!("../../assets/codes_insee_simple.json");
    serde_json::from_str::<HashMap<String, String>>(insee_codes)
}

#[component]
pub fn Dashboard(prefix: String) -> Element {
    let communes = use_resource(move || async move {
        let raw_data = reqwest::get(PROGRESS_URL)
            .await
            .unwrap()
            .json::<crate::types::Progress>()
            .await
            .unwrap()
            .features;

        let mut contribution_stats: HashMap<String, CommunePropertiesSlim> = HashMap::new();
        for commune in raw_data {
            contribution_stats.insert(commune.properties.insee.clone(), commune.properties.into());
        }
        contribution_stats
    });
    let insee_codes = read_insee_codes()?;
    let communes_data = &*communes.read();
    let have_response = communes_data.is_some();
    let mut progresses = vec![];
    if let Some(communes_data) = communes_data {
        for pair in insee_codes
            .iter()
            .filter(|pair| pair.0.starts_with(&prefix))
        {
            let code = pair.0.clone();
            let data = if let Some(data) = communes_data.get(&code) {
                data.clone()
            } else {
                CommunePropertiesSlim {
                    name: pair.1.clone(),
                    insee: code.clone(),
                    // TODO: we should be able to find this from the insee data to populate correctly
                    population: 5000,
                    contributions: 0,
                }
            };
            progresses.push(data);
        }
    }
    progresses.sort_by(|a, b| b.progress().total_cmp(&a.progress()));
    rsx! {
        div {
            class: "mx-auto w-full max-w-2xl",
            if have_response {
                DashboardSummary{progresses: progresses.clone()}
                div {
                    id: "details",
                    class: "mx-auto w-full",
                    for prog in progresses {
                        Commune{data: prog}
                    }
                }
            } else {
                div { h2{"Loading..." }}
                }
        }
    }
}
