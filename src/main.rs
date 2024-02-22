use lazy_static::lazy_static;
use std::error::Error;
use std::fs;
use std::path::Path;
use tera::Tera;
use tera::{Context, Result};

// Docs
// https://github.com/Keats/tera/blob/master/examples/basic/main.rs

const OUT_DIR: &str = "out_dir";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

fn main() {
    let mut context = Context::new();

    context.insert("params_variable", &"profile_user_account");

    let template_name = "apply_jurors.rs";
    let save_directory = "profile_validation";

    match TEMPLATES.render(template_name, &context) {
        Ok(s) => {
            let directory_path = format!("{}/{}", OUT_DIR, save_directory);

            if let Err(err) = fs::create_dir_all(directory_path.clone()) {
                eprintln!("Error creating directory: {}", err);
                ::std::process::exit(1);
            }
            let file_path = Path::new(&directory_path).join(template_name);

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
