# Neskimo Development Guidelines

## Build/Test Commands
- Build: `cargo build` or `cargo build --release`
- Run: `cargo run -- <ROM>` (e.g., `cargo run -- test_roms/nestest/nestest.nes`)
- Test all: `cargo test`
- Test specific: `cargo test cpu::cpu_test::test_adc`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style
- Follow Rust 2024 edition idioms
- Use `snake_case` for variables and functions
- Use `CamelCase` for types and enum variants
- Prefer traits over inheritance where possible
- Always implement `Debug` trait for custom types
- Wrap lines at approximately 80 characters
- Use `Result` for error handling with proper propagation
- Order imports: std, external crates, internal modules
- Document public APIs with meaningful comments
- Use `#[rustfmt::skip]` for tables or formatting that needs preservation
- Include tests for all new functionality