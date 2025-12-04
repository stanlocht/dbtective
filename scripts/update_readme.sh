#!/bin/bash
# Update version snippets in README.md before committing the bump

NEW_VERSION=$CZ_PRE_NEW_VERSION

# Update pre-commit version
sed -i "s/rev: v[0-9]\+\.[0-9]\+\.[0-9]\+/rev: v$NEW_VERSION/" README.md
