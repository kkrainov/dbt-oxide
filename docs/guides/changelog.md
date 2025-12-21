# Changelog and Release Workflow

## Overview

dbt-oxide uses **git-cliff** for automated changelog generation from git commits.

## Conventional Commits

All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Commit Types

| Type | Changelog Section | Example |
|------|-------------------|---------|
| `feat` | Features | `feat: add graph parallel processing` |
| `fix` | Bug Fixes | `fix: handle empty manifest` |
| `perf` | Performance | `perf: optimize node traversal` |
| `docs` | Documentation | `docs: update architecture guide` |
| `refactor` | Refactoring | `refactor: simplify graph builder` |
| `test` | Testing | `test: add unit tests for parser` |
| `chore` | Miscellaneous | `chore: update dependencies` |
| `ci` | CI/CD | `ci: add Python lint job` |

### Breaking Changes

Append `!` to type or add `BREAKING CHANGE:` in footer:

```
feat!: change graph API to async

BREAKING CHANGE: Graph.ancestors() is now async and requires await
```

## Generating Changelog

### For Maintainers

**Update changelog before release:**
```bash
# Generate changelog for new version
git cliff --tag v0.2.0 --prepend CHANGELOG.md

# Review and commit
git add CHANGELOG.md
git commit -m "chore: update changelog for v0.2.0"
```

**View unreleased changes:**
```bash
git cliff --unreleased
```

### Configuration

Changelog generation is configured in [`cliff.toml`](../../cliff.toml).

**Key settings:**
- Commit parsing rules (conventional commits)
- Grouping by type (Features, Bug Fixes, etc.)
- Output format (Keep a Changelog style)

## Release Workflow

See [CONTRIBUTING.md - Release Process](../../CONTRIBUTING.md#release-process) for detailed release steps.
