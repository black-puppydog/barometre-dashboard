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
        class: "rounded-md",
        div{
            class:"flex justify-between mb-1",
            span {
                class:"text-base font-medium text-blue-700 dark:text-white truncate font-bold",
                "{name}"
            }
            span {
                class:"text-sm font-medium text-blue-700 dark:text-white truncate",
                "{progress}%"
            }
        }
        div{
            class:"flex justify-between mb-1",
            span {
                class:"text-base font-xs text-blue-200 dark:text-white truncate",
              "{insee}"
            }
            span {
                class:"text-sm font-xs dark:text-red-200 dark:text-white truncate",
                "{data.contributions} / {max}"
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
