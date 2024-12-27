use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data({{params_variable}}: {{params_variable_type}}, set_end_period: WriteSignal<Option<u32>>) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: Option<u32> = client
        .request(
            "{{rpc_link}}_evidenceperiodendblock",
            rpc_params![{{params_variable}}],
        )
        .await
        .unwrap();
    set_end_period(result);
}

#[component]
pub fn EvidenceEndBlock({{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    let (end_period, set_end_period) = signal::<Option<u32>>(None);

    let action: Action<({{params_variable_type}}, WriteSignal<Option<u32>>), (), LocalStorage>= Action::new_unsync(
        |({{params_variable}}, set_end_period): &({{params_variable_type}}, WriteSignal<Option<u32>>)| {
            let {{params_variable}} = {{params_variable}}.clone();
            let set_end_period = set_end_period.clone();
            async move { load_data({{params_variable}}, set_end_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch(({{params_variable}}.clone(), set_end_period));
        },
        5000,
    );

    view! {
        <div>
            {move || {
                if end_period().is_some() {
                    view! {
                        <div>
                            {"Evidence Period ends: "}
                            <span id="end-period-time">{move || end_period()}</span>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div>
                            {"Evidence Period ends: "} <span id="end-period-time">
                                <Icon
                                    icon=icondata::ImSpinner6
                                    style="color: green"
                                />
                            </span>
                        </div>
                    }.into_any()
                }
            }}

        </div>
    }
}
