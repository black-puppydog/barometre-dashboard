use dioxus::prelude::*;

use crate::types::CommuneDisplayProps;
#[component]
pub fn Commune(data: CommuneDisplayProps) -> Element {
    let max = data.target_contributions();
    let name = &data.name;
    let progress = (100.0 * (data.contributions as f32 / max as f32)).round() as usize;
    let progress_class = data.progress_class();
    let color = crate::types::progress_class_to_color(progress_class);
    rsx!(
    div {
        class: "rounded-md mb-3 px-3",
        div{
            class:"relative overflow-hidden whitespace-nowrap pr-[20%]",
            details {
                summary {
                    class:"cursor-pointer hover:underline font-medium text-blue-700 font-semibold w-full overflow-hidden text-ellipsis whitespace-nowrap",
                    "{name}" },
                div {
                    class: "mt-2 text-gray-800 font-normal",
                    "Contributions au dernier baromètre :   {data.contributions_2021}",
                }
            },
            div {
                class:"text-blue-500 absolute right-0 top-0 w-[20%] overflow-hidden text-clip whitespace-nowrap",
                "{data.contributions} / {max}"
            }
        },
        // Progress bar with the background color of the progress class
        progress {
            class: "w-full h-6 rounded-full [&::-webkit-progress-value]:rounded-full [&::-webkit-progress-bar]:rounded-full [&::-webkit-progress-bar]:bg-slate-200 [&::-webkit-progress-value]:bg-{color}-400 [&::-moz-progress-bar]:bg-{color}-400",
            max: 100,
            value: progress,
            title: "{data.contributions} réponses sur {max} nécessaires\n{progress}%",
            "{progress}"
        }
    })
}
