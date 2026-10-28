use std::fs;
use std::path::Path;

#[test]
fn test_extension_manifest_valid() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("extension.toml");
    let content = fs::read_to_string(&manifest_path).expect("read extension.toml");

    let parsed: toml::Value = toml::from_str(&content).expect("parse extension.toml");
    let table = parsed.as_table().expect("manifest must be a table");

    assert_eq!(table.get("id").and_then(|v| v.as_str()), Some("foundation"));
    assert_eq!(table.get("name").and_then(|v| v.as_str()), Some("Foundation"));
    assert_eq!(table.get("schema_version").and_then(|v| v.as_integer()), Some(1));

    let lib = table.get("lib").and_then(|v| v.as_table()).expect("lib table");
    assert_eq!(lib.get("kind").and_then(|v| v.as_str()), Some("Rust"));
    assert_eq!(lib.get("version").and_then(|v| v.as_str()), Some("0.7.0"));

    let grammars = table.get("grammars").and_then(|v| v.as_table()).expect("grammars table");
    let foundation_grammar = grammars.get("foundation").and_then(|v| v.as_table()).expect("foundation grammar");
    assert!(foundation_grammar.contains_key("repository"));
    let rev = foundation_grammar.get("rev").and_then(|v| v.as_str());
    assert!(
        rev.is_some_and(|rev| !rev.is_empty()),
        "grammar rev must be set"
    );

    let language_servers = table
        .get("language_servers")
        .and_then(|v| v.as_table())
        .expect("language_servers table");
    let foundation_lsp = language_servers
        .get("foundation")
        .and_then(|v| v.as_table())
        .expect("foundation language server");
    assert_eq!(
        foundation_lsp.get("name").and_then(|v| v.as_str()),
        Some("Foundation LSP")
    );
}

#[test]
fn test_language_config_valid() {
    let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("languages")
        .join("foundation")
        .join("config.toml");
    let content = fs::read_to_string(&config_path).expect("read languages/foundation/config.toml");

    let parsed: toml::Value = toml::from_str(&content).expect("parse config.toml");
    let table = parsed.as_table().expect("config must be a table");

    assert_eq!(table.get("name").and_then(|v| v.as_str()), Some("Foundation"));
    assert_eq!(table.get("grammar").and_then(|v| v.as_str()), Some("foundation"));

    let suffixes = table.get("path_suffixes").and_then(|v| v.as_array()).expect("path_suffixes array");
    let has_fdn = suffixes.iter().any(|v| v.as_str() == Some("fdn"));
    assert!(has_fdn, "path_suffixes must contain 'fdn'");
}

#[test]
fn test_command_builder() {
    let cmd = foundation_zed::FoundationExtension::build_command("/usr/local/bin/foundation".to_string());
    assert_eq!(cmd.command, "/usr/local/bin/foundation");
    assert_eq!(cmd.args, vec!["lsp".to_string()]);
    assert!(cmd.env.is_empty());
}
