use crate::components::schelling_game::{{module_name}}::commit_vote_sign_in::SignTransaction;
{% if get_period %}{{ get_period.0 }}{% endif %}
{% if change_period %}{{ change_period.0 }}{% endif %}
{% if commit_end_block %}{{ commit_end_block.0 }}{% endif %}
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn CommitVote({{params_variable}}: {{params_variable_type}}) -> impl IntoView {


    // gloo::console::log!({{params_variable}}());
    let (current_view, set_current_view) = create_signal(View::Form);
    let (hash, set_hash) = create_signal::<Result<Option<[u8; 32]>, ErrorString>>(Ok(None));
    let (commit_vote, set_commit_vote) = create_signal(String::from(""));
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        if !commit_vote().is_empty() {
            let hash_data = sp_core_hashing::keccak_256(commit_vote().as_bytes());
            set_hash(Ok(Some(hash_data)));
        } else {
            panic!("Commit vote is empty");
        }

        set_current_view(View::Success);
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div  class="max-w-5xl mx-auto max-md:mx-10">
                {% if get_period %}{{ get_period.1 }}{% endif %}
                {% if commit_end_block %}{{ commit_end_block.1 }}{% endif %}
                {% if change_period %}{{ change_period.1 }}{% endif %}
                <div>
                {% if commit_vote_msg %}{{ commit_vote_msg }}{% endif %}                
                </div>
                    <form
                       
                        id="commit-vote-submit-from"
                        on:submit=submit_click
                    >
                        <div class="mb-5">
                            <label
                                for="commit-vote"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Commit Vote
                            </label>
                            <input
                                type="text"
                                id="commit-vote"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |ev| set_commit_vote(event_target_value(&ev))
                            />
                        </div>
                        <button
                            type="submit"
                            id="commit-vote-submit"
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
                        hash=hash().unwrap().unwrap()
                        {{params_variable}}={{params_variable}}.clone()
                    />

                </div>
            }
        }
       
    };

    view! {
        <>
            {move || render_view()}
        </>
    }
}
