# Contributing

## Project Structure

The implementation is split into five crates:

1. nix-libstore: Core Nix data structures
  - Want to use `nix-compat` from [snix] but they only support Nix 2.3 which
    doesn't include content-addressed derivations and dynamic derivations.

2. nix-tool: Helper to spawn Nix commands
  - In the future, we can try generating derivations without depending on
    recursive nix by implementing `nix store add` and `nix derivation add` in
    Rust.

3. nix-ninja-task: Nix derivation builder for `nix-ninja`
  - Since our generated derivations don't depend on `stdenv.mkDerivation` it is
    easier to maintain a builder as a Rust binary.
  - Prepares the source directory, runs the ninja build rule, and copies
    build outputs into Nix placeholders.

4. deps-infer: Dependency inference for C/C++
  - Ninja depends on gcc to infer header includes but in Nix we need to know
    explicitly all the header inputs upfront.
  - Parses all the include flags from the ninja build rule
  - BFS scans in threads for an include regex to infer header includes
  - Will include more headers than gcc because we match everything instead of
    processing ifdef, etc.

5. nix-ninja: Ninja compatible build system using Nix backend
  - Translate Ninja build graph to Nix derivations
  - Generate individual derivations for each build target

## Implementation notes

- See [todo][./docs/todo.md] for remaining work.
- See `TODO:` comments in code for open questions.
