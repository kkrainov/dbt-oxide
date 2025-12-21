# Upstream Tracking

This document tracks dbt-oxide's relationship with upstream dbt-core.

---

## Fork Information

- **Upstream Repository**: [dbt-labs/dbt-core](https://github.com/dbt-labs/dbt-core)
- **Fork Point**: `v1.10.16`
- **Divergence Strategy**: API compatibility, not code parity

---

## Divergence Areas

### Replaced with Rust

| Component | Status | Original | dbt-oxide |
|-----------|--------|----------|-----------|
| Graph algorithms | Complete | `networkx` (Python) | `petgraph` (Rust) |
| Manifest parsing | Complete | Python JSON | Rust `serde_json` |
| Data structures | In progress | Python dicts/lists | Rust zero-copy structs |

### Maintained from Upstream

| Component | Strategy |
|-----------|----------|
| CLI interface | Cherry-pick bug fixes |
| Jinja2 templating | Cherry-pick bug fixes |
| Adapter interface | Track API changes only |
| SQL compilation | Will replace in Phase 3 |

---

## Sync Policy

### When to Sync

- **Security fixes**: Cherry-pick immediately
- **Critical bugs**: Cherry-pick if relevant to dbt-oxide
- **API changes**: Track to maintain compatibility
- **Implementation changes**: Ignore (different architecture)
- **New features**: Evaluate case-by-case

### How to Sync

```bash
# Add upstream remote (one-time)
git remote add upstream https://github.com/dbt-labs/dbt-core.git

# Fetch upstream changes
git fetch upstream

# Review changes in a specific version
git log --oneline v1.10.16..v1.11.0

# Cherry-pick specific commits
git cherry-pick <commit-hash>

# Never merge wholesale
# git merge upstream/main  # DON'T DO THIS
```

---

## API Compatibility Testing

To ensure compatibility with the dbt ecosystem:

1. **Functional tests** - Keep passing
   - Tests in `tests/functional/` verify API contracts
   - If these pass, adapters and packages should work

2. **Adapter tests** - Run periodically
   - Test against dbt-postgres, dbt-snowflake, etc.
   - Verify adapters work with dbt-oxide

3. **Package tests** - Test popular packages
   - dbt_utils, dbt_expectations, etc.
   - Ensure packages work without modification

---

## Breaking from Upstream

### Intentional Divergences

1. **Performance optimizations** - Rust core is fundamentally different
2. **Zero-copy manifest** - Architecture differs from upstream
3. **Parallel execution** - Using Rayon instead of threading

---

## Future Upstream Sync Plan

### Quarterly Review (every 3 months)

1. Review new dbt-core releases
2. Identify relevant security/bug fixes
3. Cherry-pick or re-implement for dbt-oxide
4. Document in this file

### Annual Architecture Review (yearly)

1. Assess if upstream divergence is manageable
2. Evaluate if dbt-oxide should remain a fork
3. Consider upstreaming Rust optimizations to dbt-core

---

## Questions?

If you're unsure whether to sync a change from upstream:
- **Is it a security fix?** → Yes, cherry-pick immediately
- **Is it a bug fix in code you replaced?** → No, ignore
- **Is it a new feature?** → Evaluate business value
- **Is it an API change?** → Track and adapt

When in doubt, open a GitHub discussion.

---

**Last Updated:** 2025-12-21
