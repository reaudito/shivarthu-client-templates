use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use polkadot::runtime_types::pallet_support::Content;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(post_cid: String, {{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    view! { <ExtensionSignIn post_cid=post_cid {{params_variable}}={{params_variable}}/> }
}

#[component]
pub fn ExtensionSignIn(post_cid: String, {{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    let (account_load, set_account_load) = signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load=set_account_load/>
                </div>
            }.into_any()
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        post_cid=post_cid.clone()
                        {{params_variable}}={{params_variable}}.clone()
                        account_address=account_load().0
                        account_source=account_load().1
                    />
                </div>
            }.into_any()
        } else {
            view! { <div>{"Some Error Occured"}</div> }.into_any()
        }
    };
    view! { <div>{move || render_html()}</div> }
}

async fn transaction(
    post_cid: String,
    {{params_variable}}: {{params_variable_type}},
    account_address: String,
    account_source: String,
    set_error:WriteSignal<String>,
    set_extrinsic_success:WriteSignal<String>
) {
    let content: Content = Content::IPFS(post_cid.as_bytes().to_vec());

    {% if params_type is containing("account") %}
    let account_id32 = AccountId32::from_str(&{{params_variable}}.clone()).unwrap();

    let tx = polkadot::tx()
        .{{template_function_name}}()
        .challenge_profile(account_id32, content);
    {% endif %}

    {% if params_type is containing("number") %}

    let tx = polkadot::tx()
    .{{template_function_name}}()
    .challenge_profile({{params_variable}}, content);
    {% endif %}

    sign_in_with_extension(
        tx,
        account_address,
        account_source,
        set_error,
        set_extrinsic_success,
    )
    .await;
}

#[component]
pub fn ExtensionTransaction(
    post_cid: String,
    {{params_variable}}: {{params_variable_type}},
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = signal(String::from(""));
    let (extrinsic_success, set_extrinsic_success) = signal(String::from(""));
    let transaction_resource = create_local_resource(
        move || transaction
            (
                post_cid.clone(),
                {{params_variable}}.clone(),
                account_address.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_success,
            )
    );
        
    {{extrinsic_load}}
}
