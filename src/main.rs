use shivarthu_client_templates::modules::department_funding::department_funding;
use shivarthu_client_templates::modules::positive_externality_validation::positive_externality_validation;
use shivarthu_client_templates::modules::profile_validation::profile_validaton;
use shivarthu_client_templates::modules::project_tips::project_tips;

// Docs
// https://github.com/Keats/tera/blob/master/examples/basic/main.rs

fn main() {
    profile_validaton();
    department_funding();
    positive_externality_validation();
    project_tips();
}
