use std::{ffi::OsStr, path::Path};

fn extract_version_from_cargo_toml_content(content: &str, match_: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with(match_) {
            return line.split('"').nth(1).map(|s| s.to_string());
        }
    }
    None
}

fn extract_versions_from_readme_content(content: &str) -> Vec<String> {
    let mut versions = Vec::new();
    for line in content.lines() {
        if line.contains("tag = \"") {
            if let Some(tag_version) = line.split('"').nth(3) {
                let version = tag_version
                    .strip_prefix('v')
                    .expect("Tag version should start with 'v'")
                    .to_string();
                versions.push(version.to_string());
            }
        }
    }
    versions
}

#[derive(Debug)]
struct LintDirectory {
    pub(crate) name: String,
    pub(crate) path: std::path::PathBuf,
    pub(crate) library_name: String,
}

/// Lint directories iterator.
struct LintDirectories(Vec<LintDirectory>);

impl LintDirectories {
    fn new() -> Self {
        let lints_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("lints");
        let lints_libraries_dirs =
            std::fs::read_dir(lints_dir).expect("Failed to read lints directory");
        let mut lints_directories: Vec<LintDirectory> = Vec::new();
        for entry in lints_libraries_dirs {
            if let Ok(entry) = entry {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                let library_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .expect("Failed to get library name")
                    .to_string();

                let lint_dirs = std::fs::read_dir(&path).expect("Failed to read lint directory");
                for lint_entry in lint_dirs {
                    if let Ok(lint_entry) = lint_entry {
                        let path = lint_entry.path();
                        if path.file_name() == Some(OsStr::new("src")) {
                            // Skip the `src` directory itself
                            continue;
                        }
                        let lint_dir_path = std::fs::canonicalize(path).unwrap();

                        if lint_dir_path.is_dir() {
                            let lint_name = lint_dir_path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .expect("Failed to get lint name")
                                .to_string();
                            lints_directories.push(LintDirectory {
                                name: lint_name,
                                path: lint_dir_path,
                                library_name: library_name.clone(),
                            });
                        }
                    }
                }
            }
        }
        Self(lints_directories)
    }
}

impl Iterator for LintDirectories {
    type Item = LintDirectory;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// The README.md file contains examples about how to configure the lints.
/// Versions of these examples should match the version of the crate.
#[test]
fn version_is_updated_in_readme() {
    let main_cargo_toml_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("Cargo.toml");
    let main_cargo_toml_content =
        std::fs::read_to_string(&main_cargo_toml_path).expect("Failed to read Cargo.toml");
    let expected_version =
        extract_version_from_cargo_toml_content(&main_cargo_toml_content, "version =")
            .expect("Failed to extract version from Cargo.toml");
    let readme_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("README.md");
    let readme_content = std::fs::read_to_string(&readme_path).expect("Failed to read README.md");

    let readme_versions = extract_versions_from_readme_content(&readme_content);
    let number_of_readme_versions = readme_versions.len();
    if number_of_readme_versions != 3 {
        panic!(
            "Expected exactly two versions in README.md, found {:?}",
            number_of_readme_versions
        );
    }

    for version in readme_versions {
        if version != expected_version {
            panic!(
                "Version in README.md ({}) does not match Cargo.toml version ({})",
                version, expected_version
            );
        }
    }
}

/// Lints declare a documentation comment inside the macro provided by dylint_linting.
///
/// These comments are used to generate the READMEs for the lints. This test will
/// fail if the documentation comment is not updated, but will pass on a second
/// run, when the documentation comment is updated.
#[test]
fn lints_readmes_are_updated() {
    fn extract_lint_lib_doc_comment(lint_lib_rs_content: &str) -> Option<String> {
        let mut in_doc_comment = false;
        let mut doc_comment = String::new();

        for line in lint_lib_rs_content.lines() {
            if line.trim().starts_with("///") {
                if !in_doc_comment {
                    in_doc_comment = true;
                }

                let new_line = &line.trim().trim_start_matches("///");
                if new_line.is_empty() {
                    doc_comment.push('\n');
                } else {
                    doc_comment.push_str(&new_line[1..]);
                    doc_comment.push('\n');
                }
            } else if in_doc_comment {
                break; // End of the documentation comment
            }
        }

        if doc_comment.is_empty() {
            None
        } else {
            Some(doc_comment.trim().to_string())
        }
    }

    let mut updated = vec![];
    for LintDirectory {
        name,
        path,
        library_name,
    } in LintDirectories::new()
    {
        let lint_lib_rs_path = path.join("src").join("lib.rs");
        let readme_path = path.join("README.md");
        let readme_content = std::fs::read_to_string(&readme_path)
            .unwrap_or_else(|_| panic!("Failed to read lib/{library_name}/{name}/README.md",));
        let lint_lib_rs_content = std::fs::read_to_string(&lint_lib_rs_path)
            .unwrap_or_else(|_| panic!("Failed to read lib/{library_name}/{name}/src/lib.rs",));

        let doc_comment = extract_lint_lib_doc_comment(&lint_lib_rs_content).unwrap_or_else(|| {
            panic!(
                "Failed to extract documentation comment from lib/{library_name}/{name}/src/lib.rs",
            )
        });

        let warning_comment = "<!-- This file has been autogenerated. Don't edit it!\n\
            Instead, edit the documentation comment in the lint's src/lib.rs file. \
            -->";
        let expected_readme_content = format!("{warning_comment}\n\n# {name}\n\n{doc_comment}\n",);

        if readme_content != expected_readme_content {
            std::fs::write(&readme_path, &expected_readme_content)
                .unwrap_or_else(|_| panic!("Failed to write lib/{library_name}/{name}/README.md",));
            updated.push(name);
        }
    }

    let in_ci = std::env::var("CI").is_ok();
    let message = if in_ci {
        format!(
            "The next lints READMEs are not updated: {updated:?}. \
              Run `cargo test -p tests` locally to update them and commit the changes."
        )
    } else {
        format!(
            "The next lints READMEs have been updated: {updated:?}. \
             If you run again this test, it should pass."
        )
    };

    assert!(updated.is_empty(), "{message}");
}

/// Ensure that each lint has a help link pointing to its README.md file.
#[test]
fn lints_have_help_link() {
    for LintDirectory {
        name,
        path,
        library_name,
    } in LintDirectories::new()
    {
        let lint_lib_rs_path = path.join("src").join("lib.rs");
        let lint_lib_rs_content = std::fs::read_to_string(&lint_lib_rs_path)
            .unwrap_or_else(|_| panic!("Failed to read lints/{library_name}/{name}/src/lib.rs",));

        assert!(
            lint_lib_rs_content
                .to_lowercase()
                .contains("for further information visit"),
            "Lint help does not contains the sentence \"for further information visit\" at file {}.",
            lint_lib_rs_path.display(),
        );

        let expected_link = format!(
            "https://github.com/mondeja/rs-apps-lints/tree/main/lints/{library_name}/{name}#readme"
        );
        assert!(
            lint_lib_rs_content.contains(&expected_link),
            "Lint help does not contains the link to README.md at file {}. Expected link: {}",
            lint_lib_rs_path.display(),
            expected_link,
        );
    }
}
