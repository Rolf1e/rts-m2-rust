use web_sys::HtmlInputElement;
use yew::prelude::*;

enum AIUploadTabs {
    TypeIn,
    LoadPastebin,
    LoadGist,
}

#[function_component(TypeInAITab)]
fn type_in_ai_tab() -> Html {
    let code = use_state(|| "".to_string());

    let on_code_change = {
        let code = code.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            code.set(input.value());
        })
    };

    html! {
        <>
            <p>{ "Type in your AI's code." }</p>
            <textarea onchange={on_code_change} value={(*code).clone()} cols="80" rows="30"/>
            <div>
                <button>{"Submit AI"}</button>
            </div>
        </>
    }
}

#[function_component(LoadPastebinAITab)]
fn load_pastebin_ai_tab() -> Html {
    let pastebin_key = use_state(|| "".to_string());

    let on_pastebin_key_change = {
        let pastebin_key = pastebin_key.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            pastebin_key.set(input.value());
        })
    };

    html! {
        <>
            <p>{ "Type in your AI's Pastebin key." }</p>
            <label>{ "Key:" }<input onchange={on_pastebin_key_change} value={(*pastebin_key).clone()}/></label>
            <div>
                <button>{"Submit AI"}</button>
            </div>
        </>
    }
}

#[function_component(LoadGistAITab)]
fn load_gist_ai_tab() -> Html {
    let gist_username = use_state(|| "".to_string());
    let gist_hash = use_state(|| "".to_string());

    let on_gist_username_change = {
        let gist_username = gist_username.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            gist_username.set(input.value());
        })
    };

    let on_gist_hash_change = {
        let gist_hash = gist_hash.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            gist_hash.set(input.value());
        })
    };

    html! {
        <>
            <p>{ "Type in your AI's Gist username and hash." }</p>
            <div>
                <label>{ "Username:" }<input onchange={on_gist_username_change} value={(*gist_username).clone()}/></label>
            </div>
            <div>
                <label>{ "Hash:" }<input onchange={on_gist_hash_change} value={(*gist_hash).clone()}/></label>
            </div>
            <div>
                <button>{"Submit AI"}</button>
            </div>
        </>
    }
}

#[function_component(AIUpload)]
pub fn ai_upload() -> Html {
    let selected_tab_index = use_state(|| AIUploadTabs::TypeIn);

    let select_type_in_tab = {
        let selected_tab_index = selected_tab_index.clone();
        Callback::from(move |_| selected_tab_index.set(AIUploadTabs::TypeIn))
    };
    let select_pastebin_tab = {
        let selected_tab_index = selected_tab_index.clone();
        Callback::from(move |_| selected_tab_index.set(AIUploadTabs::LoadPastebin))
    };
    let select_gist_tab = {
        let selected_tab_index = selected_tab_index.clone();
        Callback::from(move |_| selected_tab_index.set(AIUploadTabs::LoadGist))
    };

    html! {
        <>
            <h1>{ "AI Upload" }</h1>
            <button onclick={select_type_in_tab}>{ "Write down AI" }</button>
            <button onclick={select_pastebin_tab}>{ "Load from Pastebin" }</button>
            <button onclick={select_gist_tab}>{ "Load from Github Gist" }</button>
            <div>
                {
                    match *selected_tab_index {
                        AIUploadTabs::TypeIn => html! { <TypeInAITab/> },
                        AIUploadTabs::LoadPastebin => html! { <LoadPastebinAITab/> },
                        AIUploadTabs::LoadGist => html! { <LoadGistAITab/> },
                    }
                }
            </div>
        </>
    }
}
