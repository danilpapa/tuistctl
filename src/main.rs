use crate::service::work_space_finder::find_workspace;
use crate::service::scheme_parser::get_targets;

mod service;

fn main() {
    let workspace_path = find_workspace()
        .expect("It is impossible to find \"workspace\" file in current file system");
    let targets = get_targets(&workspace_path);
    for target in targets {
        for a in target {
            println!("{}", a);
        }
    }
}
