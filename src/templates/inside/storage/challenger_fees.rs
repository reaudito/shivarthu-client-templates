use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data({{params_variable}}: {{params_variable_type}}, set_challenger_fee: WriteSignal<Option<u128>>) {
    let account_id32 = AccountId32::from_str(&{{params_variable}}).unwrap();

    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();
    let challenger_fee_storage = polkadot::storage()
        .{{template_function_name}}()
        .registration_challenge_fee();

    let challenger_fee_value = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch_or_default(&challenger_fee_storage)
        .await
        .unwrap();
    set_challenger_fee(Some(challenger_fee_value));
}

#[component]
pub fn ChallengerFees({{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    let (challenger_fee, set_challenger_fee) = signal::<Option<u128>>(None);

    let action = create_action(
        |({{params_variable}}, set_challenger_fee): &({{params_variable_type}}, WriteSignal<Option<u128>>)| {
            let {{params_variable}} = {{params_variable}}.clone();
            let set_challenger_fee = set_challenger_fee.clone();
            async move { load_data({{params_variable}}, set_challenger_fee).await }
        },
    );

    create_effect(move |_| {
        action.dispatch(({{params_variable}}.clone(), set_challenger_fee));
    });

    view! { <div>{move || format!("{:#?}", challenger_fee())}</div> }
}
