use dioxus::prelude::*;

use crate::components::commune_progress::Commune;
use crate::components::DashboardSummary;
use crate::types::{CommuneDisplayProps, CommuneDynamicData, CommuneStaticData};
use std::collections::HashMap;

#[cfg(feature = "local-dev")]
const PROGRESS_URL: &'static str = "http://localhost/example_response.json";
#[cfg(not(feature = "local-dev"))]
const PROGRESS_URL: &'static str = "https://www.barometre-velo.fr/stats/progress.geojson";
// const PROGRESS_URL: &'static str = "https://barometre.parlons-velo.fr/api/4cds56c4sdc4c56ds4cre84c13ez8c4ezc6eza9c84ze16464cdsc1591cdzf8ez/stats/geojson";

fn read_insee_codes() -> serde_json::Result<HashMap<String, CommuneStaticData>> {
    let insee_bytes_compressed = include_bytes!("../../assets/communes_insee.cbor.zstd");
    let mut decoder =
        ruzstd::decoding::StreamingDecoder::new(insee_bytes_compressed.as_slice()).unwrap();
    let mut insee_bytes = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut insee_bytes).unwrap();
    let communes = serde_cbor::from_slice::<Vec<(String, String, bool, usize)>>(&insee_bytes)
        .unwrap()
        .into_iter()
        .map(|(code, name, is_big, previous)| {
            (
                code.clone(),
                CommuneStaticData {
                    insee: code,
                    name,
                    is_big,
                    contributions_2021: previous,
                },
            )
        });
    Ok(HashMap::from_iter(communes))
}

pub enum CommuneProgressClass {
    Qualified,
    Close,
    NonZero,
    Zero,
}

#[component]
pub fn Dashboard(prefix: String) -> Element {
    if prefix.trim().len() == 0 {
        return crate::views::Home();
    }
    let communes = use_resource(move || async move {
        let raw_data = reqwest::get(PROGRESS_URL)
            .await
            .unwrap()
            .json::<crate::types::Progress>()
            .await
            .unwrap()
            .features;

        let mut contribution_stats: HashMap<String, CommuneDynamicData> = HashMap::new();
        for commune in raw_data {
            contribution_stats.insert(commune.properties.insee.clone(), commune.properties);
        }
        contribution_stats
    });

    let static_data = read_insee_codes()?;
    let dynamic_data = &*communes.read();
    let have_response = dynamic_data.is_some();

    // Now that we have the static data (last baromètre and INSEE) we need to
    // figure out the set of insee codes that we want to display.
    // However, the set of INSEE codes is not static; communes are merged or
    // split over time, and this is also the case between 2021 and 2025.
    // So there will be communes for which there simply is no historic data
    // available. But luckily we treated this in pre-processing: for these
    // communes we simply assume a previous participation of 0.
    // So at this point we simply pull all the INSEE codes of the *static* data,
    // since it has an entry for every code, even if there are no contributions
    // for that code in the current baromètre yet.
    let mut progresses = vec![];
    if let Some(communes_data) = dynamic_data {
        progresses = static_data
            .iter()
            .filter(|(code, _)| code.starts_with(&prefix))
            .map(|(code, static_commune)| {
                if let Some(dynamic_commune) = communes_data.get(code) {
                    CommuneDisplayProps {
                        name: dynamic_commune.name.clone(),
                        insee: code.clone(),
                        is_big: dynamic_commune.population >= 5000,
                        contributions: dynamic_commune.contributions,
                        contributions_2021: static_commune.contributions_2021,
                    }
                } else {
                    CommuneDisplayProps {
                        name: static_commune.name.clone(),
                        insee: code.clone(),
                        is_big: static_commune.is_big,
                        contributions: 0,
                        contributions_2021: static_commune.contributions_2021,
                    }
                }
            })
            .collect();
    }
    // sorting in two stages because sorting by progress (which is f64) isn't
    // PartialOrd and thus can't be done by key
    progresses.sort_by_key(|p| (-p.contributions, p.name.clone()));
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
