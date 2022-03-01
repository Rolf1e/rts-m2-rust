use yew::prelude::*;

enum AIUploadTabs {
    TypeIn,
    LoadPastebin,
    LoadGist,
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
                        AIUploadTabs::TypeIn => html! { "TODO" },
                        AIUploadTabs::LoadPastebin => html! { "TODO Pastebin" },
                        AIUploadTabs::LoadGist => html! { "TODO Gist" },
                    }
                }
            </div>
        </>
    }
}

