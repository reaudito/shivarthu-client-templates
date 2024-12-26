use crate::components::schelling_game::{{module_name}}::storage::get_period_fn::get_period_fn;
use leptos::prelude::*;

#[component]
pub fn GetPeriod({{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    let period = get_period_fn({{params_variable}});
    let period_value = move || match period() {
        Some(value) => format!("Period name: {:?}", value),
        None => format!(""),
    };
    view! {
        <div>
            <code>{move || period_value()}</code>
        </div>
    }
}
