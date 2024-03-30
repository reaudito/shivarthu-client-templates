use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::{{module_name}}::apply_jurors::ApplyJurors;
{% if schelling_game_name is containing("profile-validation") %}
use crate::components::schelling_game::{{module_name}}::challenge_evidence::ChallengeEvidence;
{% endif %}
use crate::components::schelling_game::{{module_name}}::commit_vote::CommitVote;
use crate::components::schelling_game::{{module_name}}::draw_jurors::DrawJurors;
use crate::components::schelling_game::{{module_name}}::reveal_vote::RevealVote;
use crate::components::schelling_game::{{module_name}}::storage::get_period_fn::get_period_fn;
use crate::services::common_services::polkadot::runtime_types::schelling_game_shared::types::Period;
use leptos::*;
use leptos_router::*;

#[component]
pub fn SchellingGame() -> impl IntoView {
    let params = use_params_map();
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

    let account = untrack(move || {{params_variable}}());

    view! {
        <div>
            <SchellingGameComponent {{params_variable}}=account/>
        </div>
    }
}

#[component]
pub fn SchellingGameComponent({{params_variable}}: {{params_variable_type}}) -> impl IntoView {
    let {{params_variable}} = move || {{params_variable}}.clone();

    let period = get_period_fn({{params_variable}}());

    let myview = move || {
        {
            {
                // let period_read_signal = period();
                if let Some(period) = period() {
                    let view = match period {
                        Period::Evidence => view! {
                            <div>
                            {% if schelling_game_name is containing("profile-validation") %}
                                <ChallengeEvidence {{params_variable}}={{params_variable}}()/>
                            {% endif %}
                            </div>
                        },
                        Period::Staking => {
                            view! {
                                <div>
                                    <ApplyJurors {{params_variable}}={{params_variable}}()/>
                                </div>
                            }
                        }
                        Period::Drawing => view! {
                            <div>
                                <DrawJurors {{params_variable}}={{params_variable}}()/>
                            </div>
                        },
                        Period::Commit => view! {
                            <div>
                                <CommitVote {{params_variable}}={{params_variable}}()/>
                            </div>
                        },
                        Period::Vote => view! {
                            <div>
                                <RevealVote {{params_variable}}={{params_variable}}()/>
                            </div>
                        },
                        Period::Appeal => view! { <div></div> },
                        Period::Execution => {
                            view! { <div>You are in Execution phase. Get your incentives</div> }
                        }
                    };
                    view
                } else {
                    view! {
                        <div class="container">
                            <p>{format!("{:?}", period())}</p>
                            <p>{"No period"}</p>
                        </div>
                    }
                }
            }
        }
    };

    view! {
        <div>
            <Nav/>
            // {move || account()}
            // {move || format!("{:?}", period())}
            {move || myview()}
        </div>
    }
}
