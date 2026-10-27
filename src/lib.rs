use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result, Worktree};

pub struct FoundationExtension;

impl FoundationExtension {
    pub fn build_command(server_path: String) -> Command {
        Command {
            command: server_path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        }
    }
}

impl Extension for FoundationExtension {
    fn new() -> Self {
        FoundationExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let path = worktree.which("foundation").ok_or_else(|| {
            "Foundation CLI binary not found in PATH. Please install Foundation or ensure it is accessible in your PATH.".to_string()
        })?;

        Ok(Self::build_command(path))
    }
}

zed::register_extension!(FoundationExtension);
