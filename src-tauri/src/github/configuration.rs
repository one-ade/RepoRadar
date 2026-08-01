use std::path::Path;

use super::client::{gh_output, label_color, repository_reference, required, run_gh};

pub fn set_variable(path: &Path, name: &str, value: &str) -> Result<(), String> {
    set_value(path, "variable", "变量", name, value, None)
}

pub fn delete_variable(path: &Path, name: &str) -> Result<(), String> {
    delete_value(path, "variable", name, None)
}

pub fn set_secret(path: &Path, name: &str, value: &str) -> Result<(), String> {
    set_value(path, "secret", "密钥", name, value, None)
}

pub fn delete_secret(path: &Path, name: &str) -> Result<(), String> {
    delete_value(path, "secret", name, None)
}

pub fn set_environment_variable(
    path: &Path,
    environment: &str,
    name: &str,
    value: &str,
) -> Result<(), String> {
    set_value(path, "variable", "变量", name, value, Some(environment))
}

pub fn delete_environment_variable(
    path: &Path,
    environment: &str,
    name: &str,
) -> Result<(), String> {
    delete_value(path, "variable", name, Some(environment))
}

pub fn set_environment_secret(
    path: &Path,
    environment: &str,
    name: &str,
    value: &str,
) -> Result<(), String> {
    set_value(path, "secret", "密钥", name, value, Some(environment))
}

pub fn delete_environment_secret(path: &Path, environment: &str, name: &str) -> Result<(), String> {
    delete_value(path, "secret", name, Some(environment))
}

pub fn save_label(path: &Path, name: &str, color: &str, description: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let name = required(name, "标签名")?;
    let color = label_color(color)?;
    gh_output(
        &reference,
        path,
        &[
            "label",
            "create",
            name,
            "--color",
            color,
            "--description",
            description.trim(),
            "--force",
        ],
    )?;
    Ok(())
}

pub fn delete_label(path: &Path, name: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    gh_output(
        &reference,
        path,
        &["label", "delete", required(name, "标签名")?, "--yes"],
    )?;
    Ok(())
}

fn set_value(
    path: &Path,
    kind: &str,
    label: &str,
    name: &str,
    value: &str,
    environment: Option<&str>,
) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{label}值不能为空"));
    }
    let reference = repository_reference(path)?;
    let args = scoped_args(kind, "set", name, environment)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    run_gh(path, &arguments, Some(&reference), Some(value)).map(|_| ())
}

fn delete_value(
    path: &Path,
    kind: &str,
    name: &str,
    environment: Option<&str>,
) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let args = scoped_args(kind, "delete", name, environment)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_output(&reference, path, &arguments).map(|_| ())
}

fn scoped_args(
    kind: &str,
    action: &str,
    name: &str,
    environment: Option<&str>,
) -> Result<Vec<String>, String> {
    let label = if kind == "secret" {
        "密钥名"
    } else {
        "变量名"
    };
    let mut args = vec![kind.into(), action.into(), required(name, label)?.into()];
    if let Some(environment) = environment {
        args.extend(["--env".into(), required(environment, "部署环境")?.into()]);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scopes_configuration_commands_to_an_environment() {
        assert_eq!(
            scoped_args("variable", "set", "REGION", Some("production")).unwrap(),
            ["variable", "set", "REGION", "--env", "production"]
        );
        assert_eq!(
            scoped_args("secret", "delete", "TOKEN", None).unwrap(),
            ["secret", "delete", "TOKEN"]
        );
        assert!(scoped_args("secret", "set", "TOKEN", Some(" ")).is_err());
    }
}
