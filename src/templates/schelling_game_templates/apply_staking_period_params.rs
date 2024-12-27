use crate::components::schelling_game::{{module_name}}::apply_staking_period::ApplyStakingPeriod;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ApplyStakingPeriodParams() -> impl IntoView {
    let params = use_params_map();
    {% if params_type is containing("account") %}
    let {{params_variable}} = move || {
        params.with(|params| {
            params
                .get("{{params_variable}}")
                .unwrap_or_default()
        })
    };
    {% endif %}

    {% if params_type is containing("number") %}

    let {{params_variable}} = move || {
        params.with(|params| {
            params
                .get("{{params_variable}}")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    {% endif %}

    let params_value = untrack(move || {{params_variable}}());

    view! {
        <div>
            <ApplyStakingPeriod {{params_variable}}=params_value/>
        </div>
    }
}