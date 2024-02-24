use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(salt: String, choice: u128, {{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    view! { <ExtensionSignIn salt=salt choice=choice {{params_variable}}={{params_variable}}/> }
}

#[component]
pub fn ExtensionSignIn(salt: String, choice: u128, {{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    let (account_load, set_account_load) = create_signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load=set_account_load/>
                </div>
            }
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        salt=salt.clone()
                        choice=choice
                        {{params_variable}}={{params_variable}}.clone()
                        account_address=account_load().0
                        account_source=account_load().1
                    />
                </div>
            }
        } else {
            view! { <div>{"Some Error Occured"}</div> }
        }
    };

    view! { <div>{move || render_html()}</div> }
}

#[component]
pub fn ExtensionTransaction(
    salt: String,
    choice: u128,
    {{params_variable}}: {{params_variable_type}},
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = create_signal(String::from("hello"));
    let (extrinsic_success, set_extrinsic_success) = create_signal(String::from("extrinsic"));
    let transaction_resource = create_local_resource(
        move || {
            (
                salt.clone(),
                choice,
                {{params_variable}}.clone(),
                account_address.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_success,
            )
        },
        move |(
            salt,
            choice,
            {{params_variable}},
            account_address,
            account_source,
            set_error,
            set_extrinsic_success,
        )| async move {
            {% if params_type is containing("account") %}
            let account_id32 = AccountId32::from_str(&{{params_variable}}.clone()).unwrap();
            let salt_vec = salt.as_bytes().to_vec();

            let tx =
                polkadot::tx()
                    .{{template_function_name}}()
                    .reveal_vote(account_id32, choice, salt_vec);
            {% endif %}

            {% if params_type is containing("number") %}
            let salt_vec = salt.as_bytes().to_vec();

            let tx =
                polkadot::tx()
                    .{{template_function_name}}()
                    .reveal_vote({{params_variable}}, choice, salt_vec);

            {% endif %}

            sign_in_with_extension(
                tx,
                account_address,
                account_source,
                set_error,
                set_extrinsic_success,
            )
            .await;
        },
    );

    let loading = transaction_resource.loading();
    let is_loading = move || {
        if loading() {
            "Loading... Please sign with extension."
        } else {
            "Idle."
        }
    };

    view! {
        <div>
            <div>{move || transaction_resource.get()}</div>
            <div>{move || is_loading()}</div>
            <div>{move || error()}</div>
            <div>{move || extrinsic_success()}</div>
        </div>
    }
}
