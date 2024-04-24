use crate::components::schelling_game::{{module_name}}::apply_staking_period_sign_in::SignTransaction;
use crate::components::schelling_game::{{module_name}}::storage::get_period::GetPeriod;
use crate::components::schelling_game::{{module_name}}::change_period::ChangePeriod;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn ApplyStakingPeriod({{params_variable}}: {{params_variable_type}}) -> impl IntoView {

    // gloo::console::log!({{params_variable}}());
    let (current_view, set_current_view) = create_signal(View::Form);
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };


    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div
                class="max-w-5xl mx-auto max-md:mx-10"
                >
                <GetPeriod {{params_variable}}={{params_variable}}.clone() /> 
                <ChangePeriod {{params_variable}}={{params_variable}}.clone() />
                    <form
                        
                        id="apply-staking-period-from"
                        on:submit=submit_click
                    >
                        <div>Apply Staking Period</div>
                        <button
                            type="submit"
                            id="apply-juror-submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>
                    </form>
                </div>
            }
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction
                        {{params_variable}}={{params_variable}}.clone()
                    />

                </div>
            }
        }
    };

    view! {
        <div>
            {move || render_view()}
        </div>
    }
}
