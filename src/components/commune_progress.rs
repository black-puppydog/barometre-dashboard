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
            class:"relative overflow-hidden whitespace-nowrap pr-[20%]",
            span {
                class:"font-medium text-blue-700 font-bold block w-full overflow-hidden text-ellipsis whitespace-nowrap",
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
                class:"text-blue-500 absolute right-0 top-0 w-[20%] overflow-hidden text-clip whitespace-nowrap my-score",
                "{data.contributions} / {max}"
            }
        }
        progress {
            class: "w-full h-6 bg-gray-200 rounded-full",
            max: 100,
            value: progress,
            title: "{data.contributions} réponses sur {max} nécessaires\n{progress}%",
            "{data.contributions} / {max}"
        }
    })
}
