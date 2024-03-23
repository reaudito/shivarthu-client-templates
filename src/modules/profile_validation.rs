use crate::modules::common_variables::EXTRINSIC_LOAD;
use crate::modules::template::{OUT_DIR, TEMPLATES};
use std::error::Error;
use std::fs;
use std::path::Path;
use tera::{Context, Result};

// Docs
// https://github.com/Keats/tera/blob/master/examples/basic/main.rs

pub fn profile_validaton() {
    let mut context = Context::new();
    context.insert("extrinsic_load", EXTRINSIC_LOAD);

    let module_name = "profile_validation";
    let import_crates_challenge_evidence = r#"use crate::components::schelling_game::profile_validation::rpc::evidence_end_block::EvidenceEndBlock;
use crate::components::schelling_game::profile_validation::storage::challenger_fees::ChallengerFees;"#;
    let import_get_period = "use crate::components::schelling_game::profile_validation::storage::get_period::GetPeriod;";
    let import_crates_staking_end_block = r#"use crate::components::schelling_game::profile_validation::rpc::staking_end_block::StakingEndBlock;"#;
    let import_crates_change_period =
        "use crate::components::schelling_game::profile_validation::change_period::ChangePeriod;";
    let import_crates_drawing_period_end = r#"use crate::components::schelling_game::profile_validation::rpc::drawing_period_end::DrawingEndBlock;"#;
    let import_crate_commit_end_block = r#"use crate::components::schelling_game::profile_validation::rpc::commit_end_block::CommitEndBlock;"#;
    let import_crate_vote_end_block = r#"use crate::components::schelling_game::profile_validation::rpc::vote_end_block::VoteEndBlock;"#;

    let components_get_period = "<GetPeriod profile_user_account=profile_user_account.clone() />";
    let components_change_period =
        "<ChangePeriod profile_user_account=profile_user_account.clone() />";
    let components_staking_end_block =
        r#"<StakingEndBlock profile_user_account=profile_user_account.clone() />"#;
    let components_drawing_period_end =
        "<DrawingEndBlock profile_user_account=profile_user_account.clone() />";
    let components_commit_end_block =
        "<CommitEndBlock profile_user_account=profile_user_account.clone() />";
    let components_vote_end_block =
        "<VoteEndBlock profile_user_account=profile_user_account.clone() />";

    let components_challenge_evidence = r#"<EvidenceEndBlock profile_user_account=profile_user_account.clone() />
                    <ChallengerFees  profile_user_account=profile_user_account.clone() />"#;

    let commit_vote_msg = r#"
    <div class="flex justify-center items-center">
    <div class="card w-96 bg-base-100 shadow-xl">
    <div class="card-body">
    <h2 class="card-title">How to vote?</h2>
    <p> Vote format, first character can be 0 or 1, your choice, then a unique
    string or salt.
    <br />1 = Evidence given for profile are valid <br />
    0 = Evidence given for profile are invalid
    <br/>
    <br/>
    For example, <br />
    0iilzmfeofopzblgycbuiahhkptp <br />
    1psiycigusjdkfoartn <br />
    0lbjvjgzqwigattqdqglzxxdepmwnsf <br />
    </p>
    <p><b>Save the vote in safe place.</b></p>
    </div>
  </div>
  </div>"#;

    // let multiline_str = r#"This is a multiline
    // string without escaping special characters."#;
    context.insert("params_variable", &"profile_user_account");
    context.insert("params_variable_type", "String");
    context.insert("params_type", "account");
    context.insert("schelling_game_name", "profile-validation");
    context.insert("template_function_name", "profile_validation");
    context.insert("module_name", module_name);
    context.insert(
        "import_crates_challenge_evidence",
        &import_crates_challenge_evidence,
    );
    context.insert(
        "components_challenge_evidence",
        components_challenge_evidence,
    );
    context.insert(
        "change_period",
        &(import_crates_change_period, components_change_period),
    );
    context.insert("get_period", &(import_get_period, components_get_period));
    context.insert(
        "staking_end_block",
        &(
            import_crates_staking_end_block,
            components_staking_end_block,
        ),
    );
    context.insert(
        "drawing_period_end",
        &(
            import_crates_drawing_period_end,
            components_drawing_period_end,
        ),
    );
    context.insert(
        "commit_end_block",
        &(import_crate_commit_end_block, components_commit_end_block),
    );
    context.insert(
        "vote_end_block",
        &(import_crate_vote_end_block, components_vote_end_block),
    );
    context.insert("commit_vote_msg", &commit_vote_msg);

    // let template_name = "apply_jurors.rs";
    let save_directory = "profile_validation";
    let template_dir = "src/templates/schelling_game_templates";
    let template_folder = "schelling_game_templates";

    // Read the directory
    if let Ok(entries) = fs::read_dir(template_dir) {
        // Iterate over the entries
        for entry in entries {
            if let Ok(entry) = entry {
                // Check if it's a file (not a directory)
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    // Get the file name as a String
                    if let Some(file_name) = entry.file_name().to_str() {
                        let template_name = format!("{}/{}", template_folder, file_name);
                        println!("{}", template_name);
                        match TEMPLATES.render(&template_name, &context) {
                            Ok(s) => {
                                let directory_path = format!("{}/{}", OUT_DIR, save_directory);

                                if let Err(err) = fs::create_dir_all(directory_path.clone()) {
                                    eprintln!("Error creating directory: {}", err);
                                    ::std::process::exit(1);
                                }
                                let file_path = Path::new(&directory_path).join(file_name);

                                // println!("{:?}", s);
                                if let Err(err) = fs::write(&file_path, s) {
                                    eprintln!("Error writing to file: {}", err);
                                    ::std::process::exit(1);
                                }

                                println!(
                                    "Template rendered successfully. Output written to: {:?}",
                                    file_path
                                );
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                let mut cause = e.source();
                                while let Some(e) = cause {
                                    println!("Reason: {}", e);
                                    cause = e.source();
                                }
                            }
                        };
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", template_dir);
    }
}
