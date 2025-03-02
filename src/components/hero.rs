use dioxus::prelude::*;

use crate::types::{CommuneData, CommuneProperties};

const PROGRESS_URL: &'static str = "https://www.barometre-velo.fr/stats/progress.geojson";

#[component]
pub fn Commune(data: CommuneProperties) -> Element {
    let name = data.name;
    let population = data.population;
    let insee = data.insee;
    rsx!(div {
                div{"{name}    {insee}"}
            div {
    class: "w-full h-6 bg-gray-200 rounded-full dark:bg-gray-700",
                progress {
                    max: 50,
                    value: data.contributions,
                    class: "h-6 bg-blue-600 rounded-full dark:bg-blue-500",
                    style:"width: 45%"}
            }})
}

#[component]
pub fn Dashboard(prefix: String) -> Element {
    let communes = use_resource(move || async move {
        reqwest::get(PROGRESS_URL)
            .await
            .unwrap()
            .json::<crate::types::Progress>()
            .await
            .unwrap()
            .features
    });
    rsx! {
        div {
            id: "hero",
            if let Some(response) = &*communes.read() {
                div { id: "links",
                    for commune in response.iter().filter(|commune|
                        commune.properties.insee.starts_with(&prefix)) {
                            Commune{ data: commune.properties.clone()
                        }
                    }
                }
            } else {
                div { h2{"Loading..." }}
            }
        }
    }
}
