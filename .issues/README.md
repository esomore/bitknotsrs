# Temporary Issue Workspace

This folder contains temporary files for preparing Radicle issues and patches. Files here are **not committed to git** and serve as a workspace for:

## Purpose

- **Issue Drafts**: Prepare comprehensive issue descriptions before creating them in Radicle
- **Patch Descriptions**: Draft patch descriptions and commit messages
- **Templates**: Reusable templates for consistent issue/patch formatting
- **Workflow Notes**: Temporary notes during development

## Workflow

1. **Draft issues** in this folder before running `just issue-new` or `rad issue open`
2. **Prepare patch descriptions** before running `just patch-new` or `rad patch open`
3. **Use templates** for consistent formatting
4. **Clean up** temporary files after syncing with Radicle

## Integration with Radicle

- Draft issues here, then copy content to `rad issue open --description "$(cat .issues/draft.md)"`
- Prepare patch descriptions for `rad patch open`
- Use `just sync` to synchronize with Radicle network
- Use `just issues` and `just patches` to view current state

## File Organization

```
.issues/
├── README.md           # This file
├── templates/          # Reusable templates
├── drafts/            # Work-in-progress issues/patches
└── temp/              # Temporary files (auto-cleanup)
```

## Example Usage

```bash
# Draft an issue
echo "## Summary\nNew feature..." > .issues/drafts/new-feature.md

# Create issue in Radicle
rad issue open --title "New Feature" --description "$(cat .issues/drafts/new-feature.md)"

# Clean up
rm .issues/drafts/new-feature.md
```

This approach gives you the benefits of local file editing while maintaining Radicle as the source of truth for issue management.