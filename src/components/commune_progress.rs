use dioxus::prelude::*;

use crate::types::CommunePropertiesSlim;

#[component]
pub fn Commune(data: CommunePropertiesSlim) -> Element {
    let max = data.target_contributions();
    let name = data.name;
    let progress = (100.0 * (data.contributions as f32 / max as f32)).round() as usize;
    let insee = data.insee;
    rsx!(
    div {
        class: "rounded-md mb-3 px-3",
        div{
            class:"whitespace-nowrap overflow-hidden",
            span {
                class:"font-medium text-blue-700 inline-block truncate max-w-[50%] font-bold float-left",
                "{name}"
            }
            // span {
            //     class:"text-blue-300 ml-5 float-left",
            //     "{insee}"
            // }
            // span {
            //     class:"text-blue-700 mx-5 float-right inline-block",
            //     "{progress}%"
            // }
            span {
                class:"text-blue-500 float-right inline-block",
                "{data.contributions} / {max}"
            }
        }
        progress {
            class: "w-full h-6 bg-gray-200 rounded-full dark:bg-gray-700",
            max: 100,
            value: progress,
            title: "{progress}%",
            "{progress}%"
        }
    })
}
