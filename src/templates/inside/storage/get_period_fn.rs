use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::schelling_game_shared::types::Period;
use polkadot::runtime_types::sortition_sum_game::types::SumTreeName;{% if schelling_game_name is containing("profile-validation")  or schelling_game_name is containing("positive-externality")  %}
use std::str::FromStr;
use subxt::utils::AccountId32;{% endif %}
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data({{params_variable}}: {{params_variable_type}}, set_period: WriteSignal<Option<Period>>) {

    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();

     
        {% if schelling_game_name is containing("profile-validation") %}

        let account_id32 = AccountId32::from_str(&{{params_variable}}).unwrap();


        let profile_validation_block_storage = polkadot::storage()
        .{{template_function_name}}()
        .validation_block(account_id32.clone());

    let profile_validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&profile_validation_block_storage)
        .await
        .unwrap();
    if profile_validation_block.is_some() {
        let key = SumTreeName::{{sumtree}} {
            citizen_address: account_id32.clone(),
            block_number: profile_validation_block.unwrap(),
        };

        let period_storage = polkadot::storage().schelling_game_shared().period_name(key);
        let period = client
            .storage()
            .at_latest()
            .await
            .unwrap()
            .fetch(&period_storage)
            .await
            .unwrap();
        gloo::console::log!(format!("period in block: {:?}", period));
        set_period(period);
    }

        {% endif %}


        {% if schelling_game_name is containing("positive-externality") %}

        let account_id32 = AccountId32::from_str(&{{params_variable}}).unwrap();


        let validation_block_storage = polkadot::storage()
        .{{template_function_name}}()
        .validation_block(account_id32.clone());

        let validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&validation_block_storage)
        .await
        .unwrap();

        if validation_block.is_some() {
            let key = SumTreeName::{{sumtree}} {
                user_address: account_id32.clone(),
                block_number: validation_block.unwrap(),
            };
    
            let period_storage = polkadot::storage().schelling_game_shared().period_name(key);
            let period = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch(&period_storage)
                .await
                .unwrap();
            gloo::console::log!(format!("period in block: {:?}", period));
            set_period(period);
        }



        {% endif %}


        {% if schelling_game_name is containing("department-funding") %}

        let validation_block_storage = polkadot::storage()
        .{{template_function_name}}()
        .validation_block({{params_variable}});

        let validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&validation_block_storage)
        .await
        .unwrap();

        if validation_block.is_some() {
            let key = SumTreeName::{{sumtree}} {
                department_required_fund_id: {{params_variable}},
                block_number: validation_block.unwrap(),
            };
    
            let period_storage = polkadot::storage().schelling_game_shared().period_name(key);
            let period = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch(&period_storage)
                .await
                .unwrap();
            gloo::console::log!(format!("period in block: {:?}", period));
            set_period(period);
        }


        {% endif %}


        {% if schelling_game_name is containing("project-tips") %}

        let validation_block_storage = polkadot::storage()
        .{{template_function_name}}()
        .validation_block({{params_variable}});

        let validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&validation_block_storage)
        .await
        .unwrap();

        if validation_block.is_some() {
            let key = SumTreeName::{{sumtree}} {
                project_id: {{params_variable}},
                block_number: validation_block.unwrap(),
            };
    
            let period_storage = polkadot::storage().schelling_game_shared().period_name(key);
            let period = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch(&period_storage)
                .await
                .unwrap();
            gloo::console::log!(format!("period in block: {:?}", period));
            set_period(period);
        }


        {% endif %}


    
       
    
   
}

pub fn get_period_fn({{params_variable}}: {{params_variable_type}}) -> ReadSignal<Option<Period>> {
    let (period, set_period) = create_signal::<Option<Period>>(None);

    let action = create_action(
        |({{params_variable}}, set_period): &({{params_variable_type}}, WriteSignal<Option<Period>>)| {
            let {{params_variable}} = {{params_variable}}.clone();
            let set_period = set_period.clone();
            async move { load_data({{params_variable}}, set_period).await }
        },
    );

    action.dispatch(({{params_variable}}.clone(), set_period));

    period
}
