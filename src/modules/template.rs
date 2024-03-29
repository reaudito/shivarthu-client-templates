use lazy_static::lazy_static;
use tera::Tera;

// pub const OUT_DIR: &str = "out_dir";

pub const OUT_DIR: &str = "/home/amiya/Documents/workspace/shivarthu/working_directory/shivarthu-client/src/components/schelling_game";

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
