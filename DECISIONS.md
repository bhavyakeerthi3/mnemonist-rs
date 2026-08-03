# Architectural Decisions

This is the current decision log for the Port Mortem Track H submission. It
records the choices that affect the submitted Rust artifact, observable
behavior, and evaluation evidence. Superseded work logs and temporary project
phases are intentionally not kept here.

## Verification Snapshot

- The 42 preserved files in `tests/original/` match the kickoff SHA-256
  manifest: `A937FEAA87B49B97426BA6C6D8FEE9718E802262AE14B26335210CE68325D381`.
- The unchanged upstream suite reports **499 passing** assertions. One
  upstream-owned `it.skip` for suffix-array issue #196 remains pending in the
  hash-preserved test file; its two regression cases run as supplemental,
  passing tests.
- `cargo test --release` reports **224 passing** Rust tests.
- `npm run verify:submission` verifies the manifest, Rust-only artifact,
  zero-unsafe audit, JSONL protocol contract, and the preserved standalone
  test path.

The counts above are evidence, not a substitute for reproduction. The commands
and raw artifacts are linked from `README.md` and `SUBMISSION.md`.

## 1. Scope: Observable Data Structures

The port targets Mnemonist's root data-structure modules and their preserved
tests. Package entrypoints, documentation, examples, and benchmark glue are
not part of the source-to-target translation scope. This keeps the submission
centered on the hard part of the pair: observable collection behavior such as
ordering, eviction, bounded capacity, indexing, and search.

The public playground exposes 41 standalone Rust protocol modules. It is a
demonstration surface for the submitted executable, not a separate JavaScript
implementation.

## 2. Submitted Artifact: Rust-Only JSONL Protocol

`target/release/mnemonist` is the submitted executable. It is built with:

```bash
cargo build --release --no-default-features --bin mnemonist
```

The executable is a persistent JSONL protocol runner. Its normal dependency
tree excludes `napi` and `napi-derive`; `npm run verify:standalone-artifact`
checks that boundary and runs `mnemonist --version`.

Node remains the host for the unmodified JavaScript test files. It is not
linked into the release executable and the executable does not load a
JavaScript runtime.

## 3. Safe Rust First

The Rust implementation contains no handwritten `unsafe` blocks, unsafe
functions, unsafe impls, or extern declarations. `npm run check:zero-unsafe`
enforces that constraint across `src/` and Rust integration tests.

Safe standard-library and ecosystem collections are preferred over pointer
translation. The design favors reviewable ownership and explicit error paths
over reproducing JavaScript internal layouts.

## 4. Values, Identity, and Missing Results

The standalone protocol uses JSON values for JSON-safe data. Where preserved
tests require JavaScript object identity or stored `undefined`, the host
boundary supplies opaque handles. Rust owns the associated collection state,
ordering, promotion, eviction, deletion, and lookup; the host resolves opaque
values back to the original JavaScript identity.

Rust core methods use `Option` for absence. JavaScript adapters normalize that
to upstream-style `undefined` at the public API boundary. This keeps Rust APIs
idiomatic without changing observable JavaScript behavior.

## 5. Ordering and Bounded Containers

Observable order is treated as part of the contract. Implementations use
`Vec`, `VecDeque`, `IndexMap`, `IndexSet`, and ordered structures where their
iteration characteristics match the required behavior.

LRU variants keep promotion, capacity eviction, iteration order, `setpop`,
deletion, and removal in Rust. The first implementation intentionally favors a
clear MRU-first representation over a pointer-heavy translation; the decision
is documented because asymptotic performance claims must follow measured
evidence rather than an assumed internal layout.

## 6. Advanced Algorithms and Exact Semantics

Advanced structures are implemented as Rust algorithms and validated against
observable results rather than superficial API shape. The notable choices are:

- `SymSpell` uses unrestricted Damerau-Levenshtein distance and preserves
  upstream suggestion ordering and duplicate counts.
- `VPTree`, `BKTree`, and `PassjoinIndex` provide standalone string-native
  paths using `leven`; distance callbacks and non-JSON inputs remain host
  boundaries.
- Trie, suffix-array, interval, sparse, Bloom filter, bit-vector, heap, and
  map paths have Rust parity and protocol coverage appropriate to their public
  surface.
- Fixed and dynamic container growth behavior follows upstream-visible policy
  results, including the BitVector and Vector edge cases covered by preserved
  tests.

## 7. Callable and GC Boundaries Are Explicit

Rust cannot truthfully execute arbitrary JavaScript closures without embedding
a JavaScript runtime. For callback-driven APIs, the JavaScript host evaluates
the callback and supplies a normalized result to the Rust-owned operation.
This applies to custom comparators, factories, hash functions, tokenizers, and
distance functions.

`DefaultWeakMap` is similarly explicit: host `WeakRef`/`WeakMap` semantics and
finalizer scheduling remain host behavior. Rust receives an explicit `release`
protocol operation and owns the observable map state it can represent. The
artifact does not claim to implement a JavaScript garbage collector.

`evidence/standalone-boundaries.json` is the authoritative machine-readable
statement of these boundaries. They are deliberate compatibility constraints,
not hidden fallbacks.

## 8. Original Tests Stay Untouched

`tests/original/` is the judge-facing north star. The manifest verification is
cross-platform, using PowerShell on Windows and a Bash implementation on
Linux/macOS. New Rust tests, differential harnesses, and supplemental
regressions live outside the preserved upstream test directory.

The suffix-array issue #196 pending case is not removed or rewritten. It stays
as the upstream `it.skip`, while the equivalent known cases are recorded as
passing supplemental regressions.

## 9. Differential, Soak, and Performance Evidence

Example tests alone are not used as the behavioral claim. The repository keeps
three complementary evidence paths:

- Seeded differential campaigns compare upstream JavaScript modules with the
  Rust executable over synchronized operation traces.
- `npm run soak:standalone -- --requests=100000` validates a persistent Rust
  process through 100k checked requests.
- `bench/results.json` records warmed p50/p95/p99 latency samples, startup,
  persistent-process throughput, and scoped RSS measurements. The methodology
  and its limits are documented in `bench/methodology.md`.

The fuzz campaign found and corrected an implementation error: optimal-string
alignment did not match Mnemonist's unrestricted Damerau behavior for a
SymSpell transposition case. The reproduction and outcome are recorded in
`UPSTREAM_FINDINGS.md` and `fuzz/log.txt`.

## 10. Cross-Platform Reproduction

The project builds with standard Cargo commands and the Dockerfile provides an
isolated Linux build. No machine-specific Cargo target override is committed.
The optional N-API bridge is verification infrastructure; it is not required
by the submitted executable.

For the shortest judge path:

```bash
npm run verify:submission
```

For the complete recorded evidence sequence:

```bash
npm run demo:submission
```

## 11. Public Demo

Mnemo Arcade is deployed at
[mnemo-arcade-rust.vercel.app](https://mnemo-arcade-rust.vercel.app). The page
sends protocol requests to Rust serverless functions and renders their results.
Browser JavaScript is limited to presentation and request transport; collection
state transitions occur in Rust.

The demo is intentionally secondary to the reproducible artifact and evidence
commands. It gives reviewers a fast way to inspect the same standalone
protocol before running the full verification suite.
