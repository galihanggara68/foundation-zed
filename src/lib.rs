use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result, Worktree};

const BINARY_NAME: &str = "foundation";

pub struct FoundationExtension;

impl FoundationExtension {
    pub fn build_command(server_path: String) -> Command {
        Command {
            command: server_path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        }
    }

    fn resolve_command(worktree: &Worktree) -> Result<Command> {
        let path = worktree.which(BINARY_NAME).ok_or_else(|| {
            format!(
                "Could not find the `{BINARY_NAME}` binary on your PATH. Install the Foundation CLI \
                 or add it to your PATH, or set `lsp.foundation.binary.path` (together with \
                 `arguments: [\"lsp\"]`) in your settings."
            )
        })?;

        Ok(Self::build_command(path))
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
        Self::resolve_command(worktree)
    }
}

zed::register_extension!(FoundationExtension);
