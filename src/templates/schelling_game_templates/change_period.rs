use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ChangePeriod() -> impl IntoView {
    let params = use_params_map();
    let navigate = leptos_router::use_navigate();

    {% if params_type is containing("account") %}
    let {{params_variable}} = move || {
        params.with(|params| {
            params
                .get("{{params_variable}}")
                .cloned()
                .unwrap_or_default()
        })
    };
    {% endif %}

    {% if params_type is containing("number") %}

    let {{params_variable}} = move || {
        params.with(|params| {
            params
                .get("{{params_variable}}")
                .cloned()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    {% endif %}
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        navigate(
            &format!("{{schelling_game_name}}-change-period/{}", {{params_variable}}()),
            Default::default(),
        );
    };

    view! {
        <div>
            <form
                class="max-w-5xl mx-auto max-md:mx-10"
                id="apply-juror-submit-from"
                on:submit=submit_click
            >
                <button
                    type="submit"
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >

                    Change Period
                </button>
            </form>
        </div>
    }
}
