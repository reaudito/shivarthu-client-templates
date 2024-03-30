use crate::modules::common_functions::list_files_recursive;
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
    context.insert("choice_type", "u128");
    context.insert("schelling_game_name", "profile-validation");
    context.insert("template_function_name", "profile_validation");
    context.insert("module_name", module_name);
    context.insert("commit_vote_msg", &commit_vote_msg);
    context.insert("rpc_link", "profilevalidation");
    context.insert("sumtree", "ProfileValidation");

    // let template_name = "apply_jurors.rs";
    let save_directory = "profile_validation";
    let template_dir = "src/templates/schelling_game_templates";
    let template_folder = "schelling_game_templates";

    let inside_dir = "src/templates/inside";

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
                        // println!("{}", template_name);
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

                                // println!(
                                //     "Template rendered successfully. Output written to: {:?}",
                                //     file_path
                                // );
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

    let inside_dir = Path::new(inside_dir);
    if let Ok(entries) = fs::read_dir(inside_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let files = list_files_recursive(&inside_dir, &path).unwrap();
                    for (absolute_path, relative_path, file_name) in files {
                        // println!(
                        //     "Absolute: {}\nRelative: {}\nFilename: {}\n",
                        //     absolute_path, relative_path, file_name
                        // );

                        let template_name = format!("inside/{}", relative_path);

                        match TEMPLATES.render(&template_name, &context) {
                            Ok(s) => {
                                let directory_path = format!("{}/{}", OUT_DIR, save_directory);

                                let file_path_string =
                                    format!("{}/{}", directory_path, relative_path);
                                let file_path = Path::new(&file_path_string);

                                let dir_path = file_path.parent().expect("Invalid file path");

                                // Create the directory structure
                                if let Err(err) = fs::create_dir_all(&dir_path) {
                                    eprintln!("Error creating directory: {}", err);
                                    ::std::process::exit(1);
                                }

                                // println!("{:?}", s);
                                if let Err(err) = fs::write(&file_path, s) {
                                    eprintln!("Error writing to file: {}", err);
                                    ::std::process::exit(1);
                                }

                                // println!(
                                //     "Template rendered successfully. Output written to: {:?}",
                                //     file_path
                                // );
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
    }
}
