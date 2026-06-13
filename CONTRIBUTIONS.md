# Contributing to Omnisystem

Thank you for your interest in contributing to the Omnisystem autonomous enterprise platform!

---

## Getting Started

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs)
- Git
- Docker (for deployment testing)
- Kubernetes cluster (optional, for K8s testing)

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd Omnisystem

# Build the complete system
cargo build --release --all

# Run all tests
cargo test --all --release

# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all --lib
```

---

## Project Structure

All work is consolidated under `Omnisystem/`:

```
Omnisystem/
├── crates/              (2,413 unified crates - main workspace)
├── Conductor/           (Complete Conductor project)
├── infrastructure/      (Deployment configs - Helm, Terraform, K8s)
├── services/            (6 core Layer 2 services)
├── languages/           (4 self-hosting languages)
├── modules/             (Additional subsystems)
├── docs/                (146 documentation files)
├── scripts/             (Utility scripts)
├── target/              (Build artifacts)
├── Cargo.toml           (Unified workspace)
└── README.md            (Project overview)
```

---

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Follow Rust conventions and idioms
- Use `cargo fmt` to format code
- Run `cargo clippy` to catch common mistakes
- Add tests for new functionality

### 3. Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }

    #[tokio::test]
    async fn test_async_feature() {
        // Async test
    }
}
```

### 4. Verify Your Changes

```bash
# Format code
cargo fmt --all

# Check linting
cargo clippy --all --lib

# Run tests
cargo test --all --release

# Build release
cargo build --release --all
```

### 5. Commit Your Changes

```bash
git add .
git commit -m "feat: Description of your feature

More detailed explanation if needed.

Co-Authored-By: Your Name <your.email@example.com>"
```

### 6. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with a description of your changes.

---

## Code Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Keep functions focused and small
- Document public APIs with doc comments

### No Unsafe Code

We maintain **zero unsafe code** throughout the platform. Please avoid unsafe blocks unless absolutely necessary and approved by maintainers.

### Error Handling

Use `Result<T>` for fallible operations:

```rust
pub fn operation(&self) -> Result<String> {
    // operation that might fail
}
```

### Testing

- Write tests for all public functions
- Test both happy path and error cases
- Use descriptive test names
- Aim for high coverage

### Comments

- Only add comments for WHY, not WHAT
- Code should be self-documenting
- Remove commented-out code

---

## Crate Structure

Each crate should follow this standard structure:

```
crate-name/
├── Cargo.toml
├── src/
│   ├── lib.rs          (Public API)
│   ├── error.rs        (Error types)
│   ├── types.rs        (Data structures)
│   └── bin/cli.rs      (CLI entry point)
├── tests/
│   └── integration.rs
└── README.md           (Crate documentation)
```

### Cargo.toml Example

```toml
[package]
name = "crate-name"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
dashmap = { workspace = true }
tracing = { workspace = true }

[dev-dependencies]
tokio = { workspace = true, features = ["full"] }
```

---

## Documentation

### Write Good Docs

- Add doc comments to public items
- Include examples in documentation
- Link to related items
- Keep docs up-to-date with code

### Example

```rust
/// Performs operation with result
/// 
/// # Arguments
/// * `input` - The input value
/// 
/// # Returns
/// Returns a string with the result
/// 
/// # Examples
/// ```
/// let result = my_function("test")?;
/// assert_eq!(result, "test-result");
/// ```
pub fn my_function(input: &str) -> Result<String> {
    // Implementation
}
```

---

## Review Process

### What We Look For

- ✅ Code follows standards
- ✅ All tests pass
- ✅ No unsafe code
- ✅ Documentation is clear
- ✅ Commit messages are descriptive
- ✅ Changes are focused

### Before Submitting

- [ ] All tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] Linter passes (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages are clear
- [ ] No breaking changes (unless approved)

---

## Commit Messages

Use clear, descriptive commit messages:

```
feat: Add new feature

More detailed explanation of what the change does
and why it was necessary.

Co-Authored-By: Your Name <your.email@example.com>
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `test:` - Test additions
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

---

## Testing

### Unit Tests

Test individual components:

```rust
#[test]
fn test_component() {
    let component = Component::new();
    assert_eq!(component.status(), "Ready");
}
```

### Integration Tests

Test component interactions:

```rust
#[tokio::test]
async fn test_integration() {
    let service = Service::new().await.unwrap();
    let result = service.execute("command").await.unwrap();
    assert!(result.contains("success"));
}
```

### Running Tests

```bash
# All tests
cargo test --all --release

# Specific crate
cargo test -p crate-name --release

# With output
cargo test --all -- --nocapture

# With logging
RUST_LOG=debug cargo test --all -- --nocapture
```

---

## Performance

### Optimization Guidelines

- Profile before optimizing
- Use benchmarks to measure improvements
- Avoid premature optimization
- Keep code readable first

### Benchmarking

```rust
#[bench]
fn bench_operation(b: &mut Bencher) {
    b.iter(|| {
        // Code to benchmark
    });
}
```

---

## Documentation Updates

If your change affects user-facing behavior:

1. Update relevant docs in `Omnisystem/docs/`
2. Update the project README if needed
3. Add migration guides if breaking changes
4. Update example code

---

## Questions?

- Check `Omnisystem/docs/` for comprehensive documentation
- Review existing code in similar crates
- Ask in pull request comments
- Open a discussion issue

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and professional
- Welcome diverse perspectives
- Provide constructive feedback
- Help others learn and grow

---

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see [LICENSE](LICENSE)).

---

**Thank you for contributing to Omnisystem!**

Together we're building the future of enterprise computing.
