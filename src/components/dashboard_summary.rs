use dioxus::prelude::*;

use crate::types::CommuneDisplayProps;

#[component]
fn DashboardTile(number: usize, text: String, color: String) -> Element {
    rsx!(
        div {
            class: "text-center border-2 border-{color}-500 rounded-2xl m-1 sm:m-5 py-5 sm:px-5 font-semibold",
            span {
                class: "text-center text-{color}-500 text-l sm:text-4xl font-bold",
                "{number}",
            },
            br{},
            span {
                class: "text-{color}-400 text-ellipsis whitespace-nowrap",
                "{text}",
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
    rsx! {
        div{
            class: "w-full rounded-xl grid grid-cols-2 sm:grid-cols-4 m-auto my-5",
            DashboardTile { number: qualified, text: "Qualifié", color: "green" },
            DashboardTile { number: close, text: "Presque", color: "orange" },
            DashboardTile { number: non_zero, text: "Non zéro", color: "red" },
            DashboardTile { number: progresses.len(), text: "Total", color: "blue" }
        }
    }
}
