# foundation-zed

First-class language support for [Foundation](https://github.com/galihanggara68/foundation) `.fdn` Blueprint files in the [Zed editor](https://zed.dev).

## Features
- **Tree-sitter Syntax Highlighting**: Accurate, structural highlighting for entities, actions, queries, traits, and types powered by [tree-sitter-foundation](https://github.com/galihanggara68/tree-sitter-foundation).
- **Code Folding & Indentation**: Full block folding and automatic indentation inside braces `{}` and parentheses `()`.
- **Integrated Language Server (LSP)**: Automatic discovery of the `foundation` CLI binary on your `$PATH` to launch `foundation lsp` for live diagnostics, error squiggles, and hover documentation.

## Installation

### From Zed Extensions Registry
In Zed:
1. Open the command palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).
2. Search for `zed: extensions`.
3. Find **Foundation** and click **Install**.

### Local Development / Sideloading
To install this extension locally in Zed:
1. Clone this repository.
2. In Zed, open the command palette and run `zed: install dev extension`.
3. Select this folder (`foundation-zed`).

## Requirements
To enable live compiler diagnostics, hover documentation, and autocompletions, the `foundation` CLI must be reachable:
```bash
foundation --version
```

By default the extension looks for `foundation` on your `$PATH`. If your binary lives elsewhere (e.g. a project-local `bin/foundation`), point Zed at it in your settings (`~/.config/zed/settings.json`):
```json
{
  "lsp": {
    "foundation": {
      "binary": {
        "path": "/absolute/path/to/foundation",
        "arguments": ["lsp"]
      }
    }
  }
}
```
When `binary.path` is set, Zed builds the launch command itself, so `arguments` must contain the `lsp` subcommand — omitting it runs `foundation` with no arguments and the server exits immediately.

## Development

The tree-sitter queries in `languages/foundation/*.scm` are a vendored copy of the grammar repo pinned in `extension.toml` (`[grammars.foundation]`), which is the source of truth. Zed only reads queries from this extension's own language directory — never from the grammar's `queries/` folder — so the copy has to live here. These commands keep the two from drifting:

```bash
make sync-queries    # copy the pinned grammar's queries into languages/foundation
make check-queries   # fail if the vendored copy differs from the pinned grammar
```

`check-queries` runs in CI. If you change queries in `tree-sitter-foundation`, run `make sync-queries` here and commit the result; if you change them here instead, remember the change belongs upstream too.

## License

Licensed under the Apache License, Version 2.0 (the "License").
Copyright 2026 Foundation Authors.
