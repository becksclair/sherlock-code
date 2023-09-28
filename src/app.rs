use leptos::{ev::MouseEvent, *};
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct PrArgs<'a> {
    pr_url: &'a str,
}

#[component]
pub fn Editor(content: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="text-area-wrapper">
            <textarea>{move || content.get()}</textarea>
            <small>"Code review will appear here."</small>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (working, set_working) = create_signal(false);
    let (pr_url, set_pr_url) = create_signal(String::new());
    let (review_content, set_review_content) = create_signal(String::new());

    let update_pr_url = move |ev| {
        let v = event_target_value(&ev);
        set_pr_url.set(v);
    };

    let do_review_pr = move |ev: MouseEvent| {
        ev.prevent_default();
        if pr_url.get_untracked().is_empty() {
            return;
        }
        spawn_local(async move {
            set_working.set(true);
            let args = to_value(&PrArgs {
                pr_url: &pr_url.get_untracked(),
            })
            .unwrap();
            let pr_info = invoke("review_pr", args).await.as_string().unwrap();
            set_review_content.set(pr_info);
            set_working.set(false);
        });
    };

    let do_describe_pr = move |ev: MouseEvent| {
        ev.prevent_default();
        if pr_url.get_untracked().is_empty() {
            return;
        }
        spawn_local(async move {
            set_working.set(true);
            let args = to_value(&PrArgs {
                pr_url: &pr_url.get_untracked(),
            })
            .unwrap();
            let pr_desc =
                invoke("describe_pr", args).await.as_string().unwrap();
            set_review_content.set(pr_desc);
            set_working.set(false);
        });
    };

    view! {
        <main class="container-fluid main">
            <div class="grid">
                <div class="main-form">
                    <label for="firstname">
                        "Pull request URL"
                        <input
                            on:input=update_pr_url
                            type="text"
                            placeholder="https://github.com/..."
                            required
                            disabled=move || working()
                        />
                    </label>

                    <Show
                        when=move || !working()
                        fallback=|| {
                            view! { <article class="busy-view" aria-busy="true"></article> }
                        }
                    >
                        <Editor content=review_content/>
                    </Show>

                    <button
                        type="submit"
                        role="button"
                        on:click=do_describe_pr
                        disabled=move || working()
                    >
                        "Generate Description"
                    </button>
                    <button
                        type="submit"
                        role="button"
                        on:click=do_review_pr
                        disabled=move || working()
                    >
                        "Sherlock that Code!"
                    </button>
                </div>
            </div>
        </main>
    }
}
