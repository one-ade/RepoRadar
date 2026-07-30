use std::path::Path;

use super::client::{gh_output, label_color, repository_reference, required, run_gh};

pub fn set_variable(path: &Path, name: &str, value: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let name = required(name, "变量名")?;
    if value.is_empty() {
        return Err("变量值不能为空".to_owned());
    }
    run_gh(
        path,
        &["variable", "set", name],
        Some(&reference),
        Some(value),
    )?;
    Ok(())
}

pub fn delete_variable(path: &Path, name: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    gh_output(
        &reference,
        path,
        &["variable", "delete", required(name, "变量名")?],
    )?;
    Ok(())
}

pub fn set_secret(path: &Path, name: &str, value: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let name = required(name, "密钥名")?;
    if value.is_empty() {
        return Err("密钥值不能为空".to_owned());
    }
    run_gh(
        path,
        &["secret", "set", name],
        Some(&reference),
        Some(value),
    )?;
    Ok(())
}

pub fn delete_secret(path: &Path, name: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    gh_output(
        &reference,
        path,
        &["secret", "delete", required(name, "密钥名")?],
    )?;
    Ok(())
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
