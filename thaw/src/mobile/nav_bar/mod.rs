mod theme;

use crate::{
    components::*,
    icon::*,
    use_theme,
    utils::{class_list::class_list, mount_style, StoredMaybeSignal},
    Theme,
};
use leptos::*;
pub use theme::NavBarTheme;

#[component]
pub fn NavBar(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] left_arrow: MaybeSignal<bool>,
    #[prop(optional, into)] left_text: MaybeSignal<String>,
    #[prop(optional, into)] on_click_left: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] right_text: MaybeSignal<String>,
    #[prop(optional, into)] on_click_right: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    mount_style("nav-bar", include_str!("./nav-bar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        theme.with(|theme| {
            format!(
                "--thaw-background-color: {};",
                theme.nav_bar.background_color
            )
        })
    });
    let title: StoredMaybeSignal<_> = title.into();
    let left_text: StoredMaybeSignal<_> = left_text.into();
    let right_text: StoredMaybeSignal<_> = right_text.into();

    let on_click_left = move |ev| {
        if let Some(click_left) = on_click_left.as_ref() {
            click_left.call(ev);
        }
    };

    let on_click_right = move |ev| {
        if let Some(click_right) = on_click_right.as_ref() {
            click_right.call(ev);
        }
    };

    view! {
        <div class=class_list!["thaw-nav-bar", move || class.get()] style=move || css_vars.get()>
            <If cond=MaybeSignal::derive(move || left_arrow.get() || !left_text.get().is_empty())>
                <Then slot>
                    <div class="thaw-nav-bar__left" on:click=on_click_left>
                        <If cond=left_arrow>
                            <Then slot>
                                <Icon icon=Icon::from(AiIcon::AiLeftOutlined)/>
                            </Then>
                        </If>
                        {move || left_text.get()}
                    </div>
                </Then>
            </If>
            <div class="thaw-nav-bar__center">{move || title.get()}</div>
            <If cond=MaybeSignal::derive(move || !right_text.get().is_empty())>
                <Then slot>
                    <div class="thaw-nav-bar__right" on:click=on_click_right>
                        {move || right_text.get()}
                    </div>
                </Then>
            </If>
        </div>
    }
}
