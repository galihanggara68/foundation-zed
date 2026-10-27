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
To enable live compiler diagnostics and autocompletions, ensure the `foundation` CLI is installed and available on your system `$PATH`:
```bash
foundation --version
```

## License

Licensed under the Apache License, Version 2.0 (the "License").
Copyright 2026 Foundation Authors.
