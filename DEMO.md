# Submission Demo

Record one terminal window from the repository root. The following takes about
five minutes on the reference host and uses the same commands checked by the
submission evidence rather than a separate demonstration harness.

```bash
npm run demo:submission
```

`make submission-demo` is an equivalent convenience alias where GNU Make is
available. The command visibly verifies the original test manifest, confirms
that the release artifact has no `napi` dependency, performs the zero-unsafe audit,
runs the JSONL protocol contract, and runs the preserved upstream suite. The
expected final Mocha line is `499 passing` and `1 pending`; that pending item is
the hash-preserved upstream `it.skip` for issue #196.

It then performs 3,200 deterministic operations against upstream `symspell.js`,
320 operations across upstream Stack, Queue, LRUCache, and BitVector, and a
100k-request checked trace through one persistent Rust process. Each campaign
prints its seed or measured request count and completes only when every
synchronized response agrees. The committed runs are recorded in `fuzz/log.txt`
and `evidence/soak.json`.

Finish by opening `bench/results.json` and `bench/methodology.md`. The report
contains warmed p50/p95/p99 distributions, startup, persistent JSONL-process
throughput, and the stated RSS limitations. It is intentionally not rerun in a
five-minute recording because the checked-in report identifies its host and
measurement date.

The demo should state the boundary plainly: the submitted `mnemonist` binary
is Rust-only and does not link Node or N-API. Node runs the preserved JavaScript
test files as their host. Arbitrary callbacks are answered at that host
boundary; Rust owns the standalone collection state and protocol algorithms.
