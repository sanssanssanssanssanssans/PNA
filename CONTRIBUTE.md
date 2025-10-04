# Contributing to PNA

Thanks for your interest in improving **PNA** — a tiny language that compiles to a self‑contained C++20 program.

This document explains the project layout, how to build and test, coding standards, and the checklist for adding features (lexer → parser → AST → codegen).

## TL;DR

```bash
# 1) Build the Rust compiler
cargo build

# 2) Compile a .pna file to C++
cargo run -- examples/while.pna -o out.cpp

# 3) Compile the generated C++ (any C++20 compiler)
g++ -std=c++20 out.cpp -o out

# 4) Run
./out
```

## Scope

- PNA is **compiled**, not interpreted.
- The compiler is written in **Rust** and currently emits a single `out.cpp` with an embedded minimal runtime.
- Supported language features: object blocks, property/variable assignment, arithmetic/logic, `log`, `input`, `cond`/`else`, `loop`, `while ... ended`, `break`, `continue`.
- Functions are **not** implemented yet (design proposals welcome). WASM backend is currently **removed**.

## Project layout

```
src/
  ast.rs            # AST nodes
  token.rs          # token kinds
  lexer.rs          # source → tokens
  parser.rs         # tokens → AST
  typeck.rs         # (light) semantic checks
  codegen_cpp.rs    # AST → C++ (with embedded runtime)
  main.rs           # CLI: reads .pna, writes out.cpp
examples/           # sample programs
scripts/            # smoke tests (optional)
```

The generated C++ file embeds a minimal runtime (Value type, Env, I/O, operators). There is no external runtime library to link.

## Toolchain

- Rust **stable** (latest is best)
- C++20 compiler (GCC, Clang, or MSVC); on Windows, MinGW-w64 or MSVC are fine
- `make`/`bash` optional for scripts

## Development workflow

1. **Fork** and create a feature branch (e.g., `feat/while-ended-fix`).
2. Make focused commits. Use **Conventional Commits** when possible:
   - `feat(lexer): ...`, `fix(parser): ...`, `refactor(codegen): ...`
3. Run formatting and checks:
   ```bash
   cargo fmt --all
   cargo clippy -- -D warnings
   cargo build
   ```
4. Add/adjust examples under `examples/` to exercise your change.
5. Open a PR with a clear description, motivation, and before/after behavior.

## Coding standards

- Rust 2021 edition, `cargo fmt` enforced.
- Prefer `Result<_, Box<dyn std::error::Error>>` for public functions; avoid panics in compiler paths.
- Keep modules **acyclic**: `main` → `lexer`/`parser`/`typeck`/`codegen_cpp` → `ast`/`token`.
- Keep the embedded C++ runtime **minimal** and deterministic.
- Clear error messages: point to the construct (token kind, lexeme) and the expectation.

## Testing

- Add small programs to `examples/` that cover your feature.
- Optional smoke tests:
  ```bash
  cargo run -- examples/02_cond.pna -o out.cpp && g++ -std=c++20 out.cpp -o out && ./out
  ```
- If you add pure Rust logic, consider unit tests in the relevant module.

## Adding a language feature (checklist)

1. **Tokens** (`src/token.rs`)
   - Add token kind(s) if new keywords/operators are required.
   - Update the **lexer** to recognize them.

2. **Parser** (`src/parser.rs`)
   - Extend grammar production(s); return meaningful errors.
   - Produce AST nodes from `src/ast.rs` (add/extend as needed).

3. **Type/Semantic checks** (`src/typeck.rs`)
   - Enforce simple invariants early (e.g., `while ... ended` shape).

4. **Codegen** (`src/codegen_cpp.rs`)
   - Map AST → C++.
   - If runtime helpers are needed, add them only inside the **embedded runtime** snippet.

5. **Examples & Docs**
   - Add a minimal example under `examples/`.
   - Update `README.md` if syntax is user‑visible.

## Style notes

- Prefer small helpers in codegen for readability.
- Avoid copying strings unnecessarily; pass `&str`/`&[Tok]` where possible.
- When in doubt, keep the language **simple** and **predictable**.

## CI

If you enable GitHub Actions, typical jobs are:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo build`
- Compile a few `examples/*.pna` to `out.cpp`, then `g++ -std=c++20` and run.

## Commit examples

- `feat(parser): support else branch in cond`
- `fix(codegen): correct ran/broke flags in while-ended`
- `refactor(lexer): simplify number scanning`
- `docs: clarify build steps on Windows`

## Reporting issues

Include:
- OS/toolchain versions (Rust, compiler)
- Input `.pna` that reproduces the issue
- Expected vs actual output
- If relevant, the generated `out.cpp` snippet

## License

Contributions are licensed under the project’s LICENSE (inbound=outbound). By submitting a PR you confirm you have the right to contribute the code.
