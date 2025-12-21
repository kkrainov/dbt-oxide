# Security Policy

## Supported Versions

dbt-oxide is under active development. Security updates are provided for the latest released version.

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| < latest| :x:                |

---

## Reporting a Vulnerability

If you discover a security vulnerability in dbt-oxide, please report it responsibly:

### Private Disclosure

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead, please report via:
1. **GitHub Security Advisories** (preferred): 
   - Go to https://github.com/kkrainov/dbt-oxide/security/advisories
   - Click "Report a vulnerability"

2. **Email**: Send details to the maintainer
   - Currently: Kirill Krainov
   - Check CODEOWNERS for current contact

### What to Include

Please include:
- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Impact assessment** (who/what is affected)
- **Suggested fix** (if you have one)
- **Your contact information** (for follow-up)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix timeline**: Depends on severity (see below)

### Severity Levels

| Severity | Example | Response Time |
|----------|---------|---------------|
| **Critical** | Remote code execution, data exposure | Within 7 days |
| **High** | Privilege escalation, authentication bypass | Within 14 days |
| **Medium** | Denial of service, information disclosure | Within 30 days |
| **Low** | Minor issues, hardening opportunities | Best effort |

---

## Security Considerations

### Rust Memory Safety

dbt-oxide uses Rust for core components, which provides memory safety guarantees. However:
- **FFI boundary**: PyO3 bindings require careful handling
- **Unsafe code**: Any `unsafe` blocks are reviewed carefully
- **Dependencies**: We monitor Rust crate security advisories

### Python Components

The Python wrapper layer inherits security considerations from:
- **dbt-core**: See upstream security considerations
- **Dependencies**: We use Dependabot for automated updates

### Dependencies

We track dependencies via:
- **Cargo.toml** (Rust dependencies)
- **pyproject.toml** (Python dependencies)
- **Dependabot** (automated security updates)

---

## Security Updates

Security fixes are released as:
1. **Patch version** for the current release
2. **Security advisory** published on GitHub
3. **Changelog entry** marking security fixes

---

## Scope

This security policy covers:
- ✅ dbt-oxide core code (Rust + Python)
- ✅ Build and release process
- ✅ Dependencies (direct)

This policy does NOT cover (report elsewhere):
- ❌ Database adapters (report to adapter maintainers)
- ❌ User dbt projects (not dbt-oxide responsibility)

### Upstream dbt-core Issues

If you find a security issue in upstream dbt-core:
- **Affects only dbt-core**: Report to [dbt Labs](https://github.com/dbt-labs/dbt-core/security)
- **Affects both dbt-core AND dbt-oxide**: Report to both projects
  - dbt-oxide (this repo)
  - dbt Labs (for upstream fix)

---

## Best Practices

When using dbt-oxide:
- ✅ Use the latest version
- ✅ Review security advisories before upgrading
- ✅ Keep Python and Rust toolchains up to date
- ✅ Use virtual environments for isolation
- ✅ Review your project's dependencies regularly

---

**Thank you for helping keep dbt-oxide secure!**
