use std::path::Path;

use serde::Deserialize;

use super::client::{gh_output, repository_reference, required};

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubReleaseEdit {
    tag: Option<String>,
    title: Option<String>,
    notes: Option<String>,
    notes_file: Option<String>,
    discussion_category: Option<String>,
    target: Option<String>,
    draft: Option<bool>,
    latest: Option<bool>,
    prerelease: Option<bool>,
    verify_tag: bool,
}

pub fn create_release(
    path: &Path,
    tag: &str,
    title: &str,
    notes: &str,
    draft: bool,
    prerelease: bool,
) -> Result<String, String> {
    let tag = required(tag, "Release Tag")?;
    let reference = repository_reference(path)?;
    let mut args = vec!["release", "create", tag];
    if !title.trim().is_empty() {
        args.extend(["--title", title.trim()]);
    }
    if notes.trim().is_empty() {
        args.push("--generate-notes");
    } else {
        args.extend(["--notes", notes.trim()]);
    }
    if draft {
        args.push("--draft");
    }
    if prerelease {
        args.push("--prerelease");
    }
    gh_output(&reference, path, &args)
}

pub fn download_release(
    path: &Path,
    tag: &str,
    target_dir: &str,
    pattern: &str,
) -> Result<(), String> {
    let tag = required(tag, "Release Tag")?;
    let target_dir = required(target_dir, "Release 目标目录")?;
    let reference = repository_reference(path)?;
    let mut args = vec!["release", "download", tag, "--dir", target_dir];
    if !pattern.trim().is_empty() {
        args.extend(["--pattern", pattern.trim()]);
    }
    args.push("--skip-existing");
    gh_output(&reference, path, &args).map(|_| ())
}

pub fn edit_release(path: &Path, tag: &str, edit: &GithubReleaseEdit) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let args = release_edit_args(tag, edit)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_output(&reference, path, &arguments).map(|_| ())
}

pub fn upload_release_assets(
    path: &Path,
    tag: &str,
    files: &[String],
    clobber: bool,
) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let args = release_upload_args(tag, files, clobber)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_output(&reference, path, &arguments).map(|_| ())
}

fn release_edit_args(tag: &str, edit: &GithubReleaseEdit) -> Result<Vec<String>, String> {
    if edit.notes.is_some() && edit.notes_file.is_some() {
        return Err("不能同时提供 Release 说明和说明文件".into());
    }
    let mut args = vec![
        "release".into(),
        "edit".into(),
        required(tag, "Release Tag")?.into(),
    ];
    for (flag, value, label) in [
        ("--tag", &edit.tag, "新 Release Tag"),
        ("--title", &edit.title, "Release 标题"),
        ("--notes-file", &edit.notes_file, "Release 说明文件"),
        (
            "--discussion-category",
            &edit.discussion_category,
            "讨论分类",
        ),
        ("--target", &edit.target, "目标分支"),
    ] {
        if let Some(value) = value.as_deref() {
            args.extend([flag.into(), required(value, label)?.into()]);
        }
    }
    if let Some(notes) = edit.notes.as_deref() {
        args.extend(["--notes".into(), notes.trim().into()]);
    }
    for (flag, value) in [
        ("--draft", edit.draft),
        ("--latest", edit.latest),
        ("--prerelease", edit.prerelease),
    ] {
        if let Some(value) = value {
            args.push(format!("{flag}={value}"));
        }
    }
    if edit.verify_tag {
        args.push("--verify-tag".into());
    }
    if args.len() == 3 {
        Err("至少选择一项 Release 修改".into())
    } else {
        Ok(args)
    }
}

fn release_upload_args(tag: &str, files: &[String], clobber: bool) -> Result<Vec<String>, String> {
    let mut args = vec![
        "release".into(),
        "upload".into(),
        required(tag, "Release Tag")?.into(),
    ];
    if files.is_empty() {
        return Err("至少选择一个 Release 资源文件".into());
    }
    for file in files {
        args.push(required(file, "Release 资源文件")?.into());
    }
    if clobber {
        args.push("--clobber".into());
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_every_supported_release_edit_flag() {
        let edit = GithubReleaseEdit {
            tag: Some("v2".into()),
            title: Some("Version 2".into()),
            notes: Some("Notes".into()),
            discussion_category: Some("Announcements".into()),
            target: Some("main".into()),
            draft: Some(false),
            latest: Some(true),
            prerelease: Some(false),
            verify_tag: true,
            ..GithubReleaseEdit::default()
        };
        let args = release_edit_args("v1", &edit).expect("valid edit");
        for flag in [
            "--tag",
            "--title",
            "--notes",
            "--discussion-category",
            "--target",
            "--draft=false",
            "--latest=true",
            "--prerelease=false",
            "--verify-tag",
        ] {
            assert!(
                args.iter().any(|argument| argument == flag),
                "missing {flag}"
            );
        }
    }

    #[test]
    fn validates_release_edits_and_uploads() {
        assert!(release_edit_args("v1", &GithubReleaseEdit::default()).is_err());
        let conflict = GithubReleaseEdit {
            notes: Some("inline".into()),
            notes_file: Some("notes.md".into()),
            ..GithubReleaseEdit::default()
        };
        assert!(release_edit_args("v1", &conflict).is_err());
        let notes_file = GithubReleaseEdit {
            notes_file: Some("notes.md".into()),
            ..GithubReleaseEdit::default()
        };
        assert!(
            release_edit_args("v1", &notes_file)
                .expect("valid notes file")
                .contains(&"--notes-file".into())
        );
        let args = release_upload_args("v1", &["a.zip".into(), "b.zip#Windows".into()], true)
            .expect("valid upload");
        assert!(args.contains(&"--clobber".into()));
        assert!(release_upload_args("v1", &[], false).is_err());
    }
}
