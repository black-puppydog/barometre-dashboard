use dioxus::prelude::*;

use crate::types::CommuneDisplayProps;

#[component]
fn DashboardTile(number: Element, text: Element, color: String) -> Element {
    rsx!(
        div {
            class: "text-center aspect-[3/2] mx-auto border-2 border-{color}-500 rounded-2xl py-3 w-32 m-1 px-5 font-semibold",
            span {
                class: "text-center text-{color}-600 text-xl text-4xl font-bold",
                {number},
            },
            br{},
            span {
                class: "text-center text-{color}-500  text-ellipsis whitespace-nowrap",
                {text},
            }
        },
    )
}

#[component]
pub fn DashboardSummary(progresses: Vec<CommuneDisplayProps>) -> Element {
    let qualified = progresses.iter().filter(|c| c.progress() >= 100f32).count();
    let close = progresses
        .iter()
        .filter(|c| {
            let missing = c.target_contributions() as i64 - c.contributions;
            0 < missing && missing <= 10
        })
        .count();
    let non_zero = progresses.iter().filter(|c| c.contributions > 0).count();
    let total_responses: usize = progresses.iter().map(|c| c.contributions as usize).sum();
    rsx! (
        div{
            class: "w-full grid grid-cols-2 sm:grid-cols-4 m-auto my-3 mx-auto",
            DashboardTile { number: rsx!("{qualified}"), text: rsx!("Communes", br{}, "Qualifiées"), color: "green" },
            DashboardTile { number: rsx!("{close}"), text: rsx!("Presque", br{}, "qualifiées"), color: "orange" },
            DashboardTile { number: rsx!(
                "{non_zero}",
                span{
                    class: "text-xs",
                    "/{progresses.len()}"
                }
            ), text: rsx!("Avec", br{}, "réponses"), color: "gray" },
            DashboardTile { number: rsx!("{total_responses}"), text: rsx!("Réponses", br{}, "total"), color: "blue" }
        }
    )
}
