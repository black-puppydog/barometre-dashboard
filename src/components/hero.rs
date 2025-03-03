use dioxus::prelude::*;

use crate::types::{CommuneData, CommuneProperties, CommunePropertiesSlim};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;

const PROGRESS_URL: &'static str = "https://www.barometre-velo.fr/stats/progress.geojson";

fn read_insee_codes() -> serde_json::Result<HashMap<String, String>> {
    let insee_codes = include_str!("../../assets/codes_insee_simple.json");
    serde_json::from_str::<HashMap<String, String>>(insee_codes)
}

#[component]
pub fn Commune(data: CommunePropertiesSlim) -> Element {
    let max = data.target_contributions();
    let name = data.name;
    let progress = (100.0 * (data.contributions as f32 / max as f32)).round() as usize;
    let insee = data.insee;
    rsx!(
    div {
        class: "rounded-md",
        div{
            class:"flex justify-between mb-1",
            span {
                class:"text-base font-medium text-blue-700 dark:text-white",
                "{name} ({insee})"
            }
            span {
                class:"text-sm font-medium text-blue-700 dark:text-white",
                "{progress}% ({data.contributions} / {max})"
            }
        }
        progress {
            class: "w-full h-6 bg-gray-200 rounded-full dark:bg-gray-700",
            max: 100,
            value: progress,
            "{progress}%"
        }
    })
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
            id: "hero",
            if have_response {
                div {
                    id: "links",
                    class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6",
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
