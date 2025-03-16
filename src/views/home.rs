use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    rsx! {
        form {
            onsubmit: move |event| {
                let binding = event.data.values();
                let prefix = binding.get("prefix");
                if let Some(prefix) = prefix {
                    nav.push(crate::Route::Dashboard { prefix: prefix.as_value() });
                }
            },
            class: "m-9",
            label {
                for: "prefix",
                "Préfix : "
            },
            input {
                class: "border-2 border-slate-200 mx-5",
                name: "prefix",
                placeholder: "83",
            },
            button {
                r#type: "submit",
                class: "border-2 border-slate-200 rounded-lg bg-slate-100 py-1 px-3 max-h-[30px] text-sm",
                "Filtrer"
            }
        }
    }
}
