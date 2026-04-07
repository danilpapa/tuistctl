use crate::service::option_parser::get_options;
use crate::service::work_space_finder::{find_workspace, options_file};
use crate::service::scheme_parser::get_targets;

mod service;

fn main() {
    let workspace_path = find_workspace()
        .expect("It is impossible to find \"workspace\" file in current file system");
    let targets = get_targets(&workspace_path);
    if let Some(option_file_path) = options_file() {
        let options = get_options(&option_file_path);
        
        for option in &options {
            println!("{}", option);
        }
    }
}
