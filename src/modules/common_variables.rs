pub const EXTRINSIC_LOAD: &str = r#"
let async_result = move || {
        transaction_resource
            .get()
            .as_deref()
            .map(|_| view!{<div></div>}.into_any())
            // This loading state will only show before the first load
            .unwrap_or_else(|| view! {
                <div class="alert">
                    <span class="loading loading-spinner"></span>
                    "Loading... Please sign with extension."
                </div>
            }
            .into_any())
    };
let error_fn = move || {
        if !error().is_empty() {
            view! {
                <div role="alert" class="alert alert-error">
                    {move || error()}
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    let extrinsic_success_fn = move || {
        if !extrinsic_success().is_empty() {
            view! {
                <div role="alert" class="alert alert-success">
                    {move || extrinsic_success()}
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    view! {
        <div class="md:container md:mx-auto">
            <div>{async_result}</div>
            <br/>
            <br/>
            <div>{move || error_fn()}</div>
            <br/>
            <div>{move || extrinsic_success_fn()}</div>

        </div>
    } 
"#;
