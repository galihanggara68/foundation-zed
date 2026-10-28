.PHONY: sync-queries check-queries

# Refresh the vendored tree-sitter queries in languages/foundation from the
# grammar repo pinned in extension.toml.
sync-queries:
	./scripts/sync-grammar-queries.sh sync

# Fail when the vendored queries drift from the pinned grammar.
check-queries:
	./scripts/sync-grammar-queries.sh check
