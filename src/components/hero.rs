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
    let name = data.name;
    let population = data.population;
    let max: f32 = if population < 5000 { 30.0 } else { 50.0 };
    let progress = (100.0 * (data.contributions as f32 / max)).round() as usize;
    let insee = data.insee;
    rsx!(div {
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
        div {
            class: "w-full h-6 bg-gray-200 rounded-full dark:bg-gray-700",
            div {
                class: "h-6 bg-blue-600 rounded-full dark:bg-blue-500",
                style: "width: {progress}%"
            }
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
        for pair in insee_codes.iter().filter(|pair| pair.0.starts_with("83")) {
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
                    // for commune in response.iter().filter(|commune| commune.0.starts_with("83")) {
                    // for (&code, &name) in {
                    //     div {
                    //         Commune{ data: response.get(code).or(Some(CommunePropertiesSlim{
                    //             name,
                    //             insee: code,
                    //             // TODO: we should be able to find this from the insee data to populate correctly
                    //             population: 5000,
                    //             contributions: 0,
                    //         })).unwrap().clone()
                    //         }
                    //     }
                    // }
                }
            } else {
                div { h2{"Loading..." }}
            }
        }
    }
}
