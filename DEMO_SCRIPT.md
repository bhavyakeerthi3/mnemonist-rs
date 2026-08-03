# Five-Minute Demo Script

This script is a concise walkthrough for the Port Mortem submission video. It
keeps the focus on reproducible evidence instead of broad claims.

## 0:00 - 0:25 | Problem And Goal

> Hi, I am Bhavya. This is Mnemo Arcade, my Track H JavaScript-to-Rust port of
> Mnemonist. Mnemonist is a data-structure library, so the challenge was not
> translating method names. The challenge was preserving observable behavior:
> ordering, capacity rules, eviction, indexing, and edge cases.

## 0:25 - 1:10 | Live Rust Playground

Open [Mnemo Arcade](https://mnemo-arcade-rust.vercel.app), enter the arcade,
select `Stack`, and press `PUSH`.

> This page is a live interface to the Rust protocol. The browser sends a
> request, Rust changes the collection, and the Rust State Buffer displays the
> returned state. The selector exposes 41 standalone Rust protocol modules.
> The JavaScript page renders requests and responses; collection state
> transitions happen in Rust.

Point out `RUST STATE BUFFER` and `PROTOCOL TRACE`.

## 1:10 - 2:00 | Submitted Artifact

Open the README and its At A Glance table.

> The website is a demonstration, but the submitted artifact is a Node-free
> Rust JSONL executable. It does not link N-API or a JavaScript runtime. Node
> remains only as the host for Mnemonist's unchanged JavaScript test files.
> The executable being tested is Rust.

Show:

```bash
cargo build --release --no-default-features --bin mnemonist
```

## 2:00 - 3:00 | Test Integrity And Functional Parity

Run or show the final output of:

```bash
npm run verify:submission
```

> This command verifies the kickoff hashes for all 42 original test files,
> verifies the Rust-only dependency boundary, runs the zero-unsafe audit,
> checks the JSONL protocol, and runs the preserved upstream path. The current
> result is 499 passing upstream assertions. The only pending case is Mnemonist's
> own hash-preserved `it.skip` for suffix-array issue #196; it was not deleted
> or rewritten.

## 3:00 - 4:00 | Behavioral Evidence

Open `fuzz/log.txt`, `UPSTREAM_FINDINGS.md`, or `SUBMISSION.md`.

> I added evidence beyond example tests. Seeded differential campaigns run the
> same generated operation traces against upstream JavaScript and the Rust
> executable, comparing results and state. A persistent Rust process also
> survives a checked 100,000-request soak. Rust property tests check invariants
> such as LRU reference-model ordering and BitVector rank and select behavior.

> One differential campaign found an incorrect edit-distance implementation in
> the SymSpell port. I corrected it to Mnemonist's unrestricted
> Damerau-Levenshtein behavior and recorded the reproduction.

## 4:00 - 4:35 | Rust Quality And Honest Boundaries

Open `DECISIONS.md` and `evidence/standalone-boundaries.json`.

> The Rust suite has 224 passing release tests, and the project has zero
> handwritten unsafe blocks. For arbitrary JavaScript closures and WeakMap
> garbage-collection scheduling, Rust cannot honestly recreate JavaScript
> runtime semantics without embedding that runtime. Those are explicit host
> boundaries. Rust owns the collection state and algorithms; the host supplies
> callback or GC events where JavaScript behavior is required.

## 4:35 - 5:00 | Close

> This is a reproducible Rust artifact with preserved tests, a Rust-only
> executable boundary, zero unsafe code, differential and soak evidence,
> documented decisions, benchmark methodology, and a live demo. Every claim is
> linked from the public repository so it can be audited. Thank you.

## Questions To Answer Directly

**Why does Node appear in the verification command?**

> Node hosts the original unchanged JavaScript tests. It is not linked into the
> submitted Rust executable.

**Why is there one pending test?**

> It is the original upstream `it.skip` for issue #196. The file remains
> hash-identical and the related regression cases are supplemental passing tests.

**Is every JavaScript runtime behavior inside Rust?**

> The submitted executable is Rust-only. Arbitrary JavaScript callbacks and
> host garbage-collection timing are documented host boundaries rather than
> misrepresented as Rust behavior.
