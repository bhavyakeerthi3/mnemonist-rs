# Submission Evidence

## Evidence Index

| Evidence area | Recorded proof | Reproduce or inspect |
| --- | --- | --- |
| Runnable Rust artifact | Node-free JSONL executable | `cargo build --release --no-default-features --bin mnemonist` |
| Test integrity | 42 upstream files match the kickoff manifest | `npm run verify:original-tests` |
| Functional parity | 499 passing unchanged upstream assertions, 1 preserved upstream pending | `npm run verify:submission` |
| Rust quality | 231 release tests and zero handwritten `unsafe` | `cargo test --release --no-default-features`, `npm run check:zero-unsafe` |
| Behavioral evidence | Seeded differential tests and 100k persistent-process soak | `fuzz/log.txt`, `npm run soak:standalone -- --requests=100000` |
| Performance honesty | Warmed p50/p95/p99, startup, throughput, and scoped RSS methodology | `bench/results.json`, `bench/methodology.md` |
| Architecture boundaries | Callback and GC semantics stated explicitly | `DECISIONS.md`, `evidence/standalone-boundaries.json` |
| Live inspection | 41 Rust protocol modules in the browser playground | [mnemo-arcade-rust.vercel.app](https://mnemo-arcade-rust.vercel.app) |

## Artifact

Build the submitted executable with:

```bash
cargo build --release --no-default-features --bin mnemonist
```

The result is `target/release/mnemonist`, a Rust JSONL executable. It does not
link Node or N-API. `npm run verify:standalone-artifact` enforces that the
submission dependency tree excludes `napi`, `napi-derive`, `tokio`,
`vercel_runtime`, and `http-body-util`.

## Track H Scope

This Open Pair submission ports Mnemonist's root data-structure modules from
JavaScript to Rust. It deliberately excludes upstream documentation, benchmark
material, examples, and package-entrypoint glue from the porting scope. The
reason for the pairing is behavioral: these structures expose ordering,
eviction, indexing, and search semantics that are meaningful to verify across
language boundaries, while Rust provides a native executable with explicit
ownership and no linked JavaScript runtime.

## Verification

```bash
npm run verify:submission
```

Latest checked result: all 42 original-test files match kickoff hash
`5195E46CC0451C8F285B60EFB11BD280E1543840D8C631AB331FB76BBAC43BBE`; the
standalone route reports 499 passing assertions and one unchanged upstream
`it.skip` for suffix-array issue #196. The no-unsafe audit reports zero unsafe
blocks, functions, implementations, and extern declarations.

## Behavioral Evidence

```bash
npm run fuzz:standalone-symspell -- --duration-ms=120000 --steps=800 --seed=1592639710
npm run fuzz:standalone-collections -- --duration-ms=120000 --steps=80 --seed=1592639710
npm run soak:standalone -- --requests=100000
cargo test --release --test fuzz_harness
npm run bench:distribution
```

The SymSpell campaign compares the original source module with the Rust
executable across four option modes. The collection campaign compares Stack,
Queue, LRUCache, and BitVector observable results and state after each
operation. The soak sends 100k checked requests through one persistent Rust
process, exercising long-lived protocol state without the N-API addon. A Rust
test separately runs eight independent Stack/Queue instances in parallel; it
is intentionally scoped as instance-isolation evidence, not shared-object
concurrency equivalence with JavaScript. Rust properties independently check
LRU reference-model ordering and BitVector rank/select invariants. Reproducers
include seeds and recent operation traces.

`bench/results.json` contains warmed p50/p95/p99 distributions, startup,
persistent JSONL throughput, and explicitly scoped RSS samples. The Rust RSS
probe holds a defined Stack/Queue workload live for a Windows `WorkingSet64` or
Linux `VmRSS` sample; it is a retained-working-set measurement, not peak RSS.
`bench/methodology.md` records the host, sample definition, and comparison
limits.

## Boundaries

`evidence/standalone-boundaries.json` is the authoritative machine-readable
boundary statement. Rust owns collection state and protocol algorithms. The
preserved JavaScript tests remain their own host, and arbitrary callback results
are supplied at that explicit boundary. The artifact neither links a JavaScript
runtime nor schedules its own garbage collection: the test host reports
DefaultWeakMap key collection through an explicit `release` protocol operation,
which Rust applies to its owned map state.

## Demo

```bash
npm run demo:submission
```

See `DEMO.md` for the recording sequence.
