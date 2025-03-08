use dioxus::prelude::*;

use crate::types::CommunePropertiesSlim;

#[component]
pub fn DashboardSummary(progresses: Vec<CommunePropertiesSlim>) -> Element {
    let qualified = progresses.iter().filter(|c| c.progress() >= 100f32).count();
    let close = progresses
        .iter()
        .filter(|c| {
            let missing = c.target_contributions() as i64 - c.contributions;
            0 < missing && missing <= 10
        })
        .count();
    let non_zero = progresses.iter().filter(|c| c.contributions > 0).count();
    rsx! {
        div{
            class: "w-full rounded-xl grid grid-cols-2 sm:grid-cols-4 m-auto my-5",
            div {
                class: "text-center border-2 border-green-500 rounded-2xl m-1 sm:m-5 p-5 font-semibold",
                span {
                    class: "text-center text-green-500 text-l sm:text-4xl font-bold",
                    "{qualified}",
                },
                br{},
                span {
                    class: "text-green-400",
                    "Qualifié",
                }
            },
            div {
                class: "text-center border-2 border-orange-500 rounded-2xl m-1 sm:m-5 p-5 font-semibold",
                span {
                    class: "text-center text-orange-500 text-l sm:text-4xl font-semibold",
                    "{close}"
                },
                br{},
                span {
                    class: "text-orange-400",
                    "Presque"
                }
            }
            div{
                class: "text-center border-2 border-red-500 rounded-2xl m-1 sm:m-5 p-5 font-semibold",
                span {
                    class: "text-center text-red-500 text-l sm:text-4xl font-semibold",
                    "{non_zero}"
                },
               br {}
               span {
                   class: "text-red-400",
                   "Non zéro"
               }
            },
            div{
                class: "text-center border-2 border-blue-500 rounded-2xl m-1 sm:m-5 p-5 font-semibold",
                span {
                    class: "text-center text-blue-500 text-l sm:text-4xl font-semibold",
                    "{progresses.len()}"
                },
               br {}
               span {
                   class: "text-blue-400",
                   "Total"
               }
            },
        }
    }
}
