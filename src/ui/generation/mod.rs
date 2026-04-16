use crate::service::option_parser::TuistOption;

pub fn generate_cmd(
    targets: &Vec<String>,
    options: &Vec<String>,
    source_options: &Vec<TuistOption>,
) -> String {
    let cmd: String = if !options.is_empty() {
        let tuist_options = into_options(&options, &source_options);
        generate_base_command(&targets) + " && " + generate_options_command(&tuist_options).as_str()
    } else {
        generate_base_command(&targets)
    };
    return cmd;
}

fn into_options(options: &Vec<String>, source: &Vec<TuistOption>) -> Vec<TuistOption> {
    options
        .iter()
        .map(|option_string | {
            source.iter()
                .find(|source_tuist_option| source_tuist_option.name == *option_string)
                .expect(&format!("Unknown option: {}", option_string))
                .clone()
        })
        .collect()
}

fn generate_base_command(targets: &Vec<String>) -> String {
    format!("tuist generate {}", targets.join(" "))
}

fn generate_options_command(options: &Vec<TuistOption>) -> String {
    let mut cmd: Vec<String> = Vec::new();
    let mut env_values: Vec<String> = Vec::new();
    for option in options {
        if let Some(execution) = &option.exec {
            cmd.push(execution.to_string());
        } else {
            env_values.push(option.name.to_string());
        }
    };
    for env in env_values {
        cmd.push(env.to_string() + "=1");
    }
    return cmd.join(" ");
}