#!/usr/bin/env sh
#
# Keep the vendored tree-sitter queries in sync with the grammar repo pinned in
# `extension.toml`.
#
# The grammar repo (`[grammars.foundation]`) is the source of truth for the
# queries; `languages/foundation/*.scm` is a vendored copy that Zed actually
# loads (Zed only reads queries from the extension's language directory, never
# from the grammar clone's own `queries/` folder).
#
# Usage:
#   scripts/sync-grammar-queries.sh sync    Copy the grammar's queries into languages/foundation
#   scripts/sync-grammar-queries.sh check   Exit non-zero if the vendored copy has drifted

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT_DIR"

MANIFEST=extension.toml
GRAMMAR_NAME=foundation
GRAMMAR_DIR=grammars/$GRAMMAR_NAME
QUERIES_DIR=$GRAMMAR_DIR/queries
LANG_DIR=languages/foundation

read_grammar_field() {
    awk -v section="[grammars.$GRAMMAR_NAME]" -v field="$1" '
        $0 == section { in_section = 1; next }
        /^\[/         { in_section = 0 }
        in_section && $1 == field {
            sub(/^[^=]*=[^"]*"/, "")
            sub(/".*$/, "")
            print
            exit
        }
    ' "$MANIFEST"
}

GRAMMAR_REPO=$(read_grammar_field repository)
GRAMMAR_REV=$(read_grammar_field rev)

if [ -z "$GRAMMAR_REPO" ] || [ -z "$GRAMMAR_REV" ]; then
    echo "error: could not read repository/rev for [grammars.$GRAMMAR_NAME] from $MANIFEST" >&2
    exit 1
fi

fetch_grammar() {
    if [ -d "$GRAMMAR_DIR/.git" ]; then
        git -C "$GRAMMAR_DIR" fetch --depth 1 origin "$GRAMMAR_REV" >/dev/null 2>&1 || true
    else
        mkdir -p "$GRAMMAR_DIR"
        git -C "$GRAMMAR_DIR" init --quiet
        git -C "$GRAMMAR_DIR" remote add origin "$GRAMMAR_REPO"
        git -C "$GRAMMAR_DIR" fetch --depth 1 origin "$GRAMMAR_REV"
    fi
    git -C "$GRAMMAR_DIR" checkout --quiet "$GRAMMAR_REV"
}

sync_queries() {
    [ -d "$QUERIES_DIR" ] || {
        echo "error: $QUERIES_DIR does not exist" >&2
        exit 1
    }

    for query in "$QUERIES_DIR"/*.scm; do
        [ -e "$query" ] || continue
        name=$(basename "$query")
        cp "$query" "$LANG_DIR/$name"
        echo "synced $name"
    done
}

check_queries() {
    [ -d "$QUERIES_DIR" ] || {
        echo "error: $QUERIES_DIR does not exist" >&2
        exit 1
    }

    drift=0
    for query in "$QUERIES_DIR"/*.scm; do
        [ -e "$query" ] || continue
        name=$(basename "$query")
        vendored="$LANG_DIR/$name"
        if [ ! -f "$vendored" ]; then
            echo "drift: $vendored is missing (grammar provides $name)" >&2
            drift=1
        elif ! cmp -s "$query" "$vendored"; then
            echo "drift: $vendored differs from $QUERIES_DIR/$name" >&2
            drift=1
        fi
    done

    if [ "$drift" -ne 0 ]; then
        echo "" >&2
        echo "Run 'make sync-queries' to refresh the vendored queries." >&2
        return 1
    fi

    echo "queries are in sync with $GRAMMAR_REPO@$GRAMMAR_REV"
}

case "${1:-}" in
    sync)
        fetch_grammar
        sync_queries
        ;;
    check)
        fetch_grammar
        check_queries
        ;;
    *)
        echo "usage: $0 {sync|check}" >&2
        exit 2
        ;;
esac
