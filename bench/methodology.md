# Benchmark methodology

## Environment

- OS: Windows 11 locally; Docker/Linux recommended for final repeatability
- Rust: 1.97+
- Node: latest LTS for original/reference adapter runs

## Metrics

| Metric | How measured |
|--------|--------------|
| Startup | Repeated child-process wall-clock timing over the release binary and Node modules |
| Throughput | Hot-loop stack push/pop and queue enqueue/dequeue |
| Persistent protocol throughput | One release Rust process receives a stream of JSONL Stack creation/push requests |
| RSS | Isolated Node post-workload working set and a sampled, live Rust retained-workload process |
| p99 | 20 repeated warmed workloads, report 99th percentile latency |

## Repeated Workloads

Run:

```bash
npm run bench:distribution
```

This warms each workload three times, then reports 20 samples for:

1. Rust core release binary.
2. N-API Stack/Queue adapters.
3. Preserved upstream JavaScript Stack/Queue modules.
4. Rust and Node module startup.
5. N-API and upstream-Node RSS after isolated stack/queue workloads.
6. Rust-process RSS while a dedicated Rust executable retains a 200k-item Stack
   and 200k-item Queue, sampled before the process is released.
6. Persistent JSONL protocol processing for 10k Stack pushes, including process startup.

The command refreshes `bench/results.json`. Set `BENCH_OUTPUT` to write the
same report to a different path.

Each stack and queue sample creates a collection, pushes/enqueues 200k
integers, then pops/dequeues them. The report gives min, p50, p95, p99, and
max rather than relying on a single average.

The protocol workload writes request responses to an ignored stdout sink only
for timing; `npm run test:standalone` separately verifies the response contract.
It measures the runner's persistent stdin-loop path, not the synchronous replay
adapter used by the unchanged JavaScript conformance subset. Set
`BENCH_PROTOCOL_ITERATIONS` to change its default 10k request count.

## Confounders

- Debug vs release: always use `--release`.
- JS JIT warmup: use warmup runs before comparing against Node.
- Allocator differences: Windows local numbers are useful smoke data, not final cross-platform proof.
- Native Node adapter: local Windows `.node` production is verified with the detected x64 MinGW toolchain. The per-item JSON N-API benchmark includes boundary/serialization cost, so it is not a direct substitute for Rust-core throughput.
- RSS: `process.memoryUsage().rss` is sampled after each isolated Node workload.
  A separate `rss_bench` Rust process fills and retains a 200k-item Stack and
  Queue, prints a readiness marker, and is sampled with Windows `WorkingSet64`
  or Linux `VmRSS` before release. Neither measurement is OS peak RSS. The Rust
  and Node samples have intentionally different retained-state definitions, so
  they are evidence of bounded, reproducible working sets rather than a direct
  memory-efficiency ratio.

## Results

See `results.json`.

## Interpretation Limits

- p99 is the maximum of the 20 warmed samples in this small local sample; it
  is useful for repeatable regression detection, not a population-level SLO.
- Startup and persistent-protocol measurements include process creation where
  stated in the workload name; hot-loop measurements do not.
- RSS reports post-workload Node working sets and a retained-workload Rust
  working set, not peak RSS. Do not compare the values as a ratio because the
  samples intentionally retain different collection states.
- Results are local Windows measurements. Compare each runtime's distribution
  and methodology, not isolated throughput ratios across runtime boundaries.
