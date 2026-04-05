use crate::service::path_finder::find_workspace;

mod service;

fn main() {
    let workspace_file = find_workspace()
        .expect("It is impossible to find \"workspace\" file in current file system");
    println!("{}", workspace_file.display());
}
