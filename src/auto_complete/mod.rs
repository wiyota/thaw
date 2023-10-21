use crate::{mount_style, teleport::Teleport, utils::maybe_rw_signal::MaybeRwSignal, Input};
use leptos::*;

#[derive(Clone, PartialEq)]
pub struct AutoCompleteOption {
    pub label: String,
    pub value: String,
}

#[component]
pub fn AutoComplete(
    #[prop(optional, into)] value: MaybeRwSignal<String>,
    #[prop(optional, into)] placeholder: MaybeSignal<String>,
    #[prop(optional, into)] options: MaybeSignal<Vec<AutoCompleteOption>>,
) -> impl IntoView {
    mount_style("auto-complete", include_str!("./auto-complete.css"));
    let is_show_menu = create_rw_signal(false);
    let auto_complete_ref = create_node_ref::<html::Div>();
    let auto_complete_menu_ref = create_node_ref::<html::Div>();
    let show_menu = move || {
        is_show_menu.set(true);
        let rect = auto_complete_ref
            .get_untracked()
            .unwrap()
            .get_bounding_client_rect();

        let auto_complete_menu = auto_complete_menu_ref.get_untracked().unwrap();
        auto_complete_menu
            .style("width", format!("{}px", rect.width()))
            .style(
                "transform",
                format!(
                    "translateX({}px) translateY({}px)",
                    rect.x(),
                    rect.y() + rect.height()
                ),
            );
    };

    let allow_value = move |_| {
        if !is_show_menu.get_untracked() {
            show_menu();
        }
        true
    };

    view! {
        <div class="melt-auto-complete" ref=auto_complete_ref>
            <Input
                value
                placeholder
                on_focus=move |_| show_menu()
                on_blur=move |_| is_show_menu.set(false)
                allow_value
            />
        </div>
        <Teleport>
            <div
                class="melt-auto-complete__menu"
                style=move || {
                    if is_show_menu.get() { None } else { Some("display: none;") }
                }

                ref=auto_complete_menu_ref
            >

                {move || {
                    options
                        .get()
                        .into_iter()
                        .map(|v| {
                            let AutoCompleteOption { value: option_value, label } = v;
                            let on_click = move |_| {
                                value.set(option_value.clone());
                                is_show_menu.set(false);
                            };
                            let on_mousedown = move |ev: ev::MouseEvent| {
                                ev.prevent_default();
                            };
                            view! {
                                <div
                                    class="melt-auto-complete__menu-item"
                                    on:click=on_click
                                    on:mousedown=on_mousedown
                                >
                                    {label}
                                </div>
                            }
                        })
                        .collect_view()
                }}

            </div>
        </Teleport>
    }
}