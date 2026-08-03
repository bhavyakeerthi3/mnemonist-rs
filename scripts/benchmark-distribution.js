'use strict';

const { execFileSync, spawnSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const OriginalStack = require('../original/mnemonist/stack.js');
const OriginalQueue = require('../original/mnemonist/queue.js');
const NativeStack = require('../tests/stack.js');
const NativeQueue = require('../tests/queue.js');

const root = path.resolve(__dirname, '..');
const runs = Number(process.env.BENCH_RUNS || 20);
const warmups = Number(process.env.BENCH_WARMUPS || 3);
const iterations = 200000;
const protocolIterations = Number(process.env.BENCH_PROTOCOL_ITERATIONS || 10000);
const executable = process.platform === 'win32' ? '.exe' : '';
const rustBench = path.join(root, 'target', 'release', `bench${executable}`);
const rustDemo = path.join(root, 'target', 'release', `mnemonist${executable}`);

function measured(workload) {
  for (let i = 0; i < warmups; i++) workload();
  const samples = [];
  for (let i = 0; i < runs; i++) samples.push(workload());
  return summary(samples);
}

function summary(samples) {
  const ordered = samples.slice().sort((a, b) => a - b);
  const percentile = ratio => ordered[Math.min(ordered.length - 1, Math.ceil(ordered.length * ratio) - 1)];
  return {
    samples_us: samples,
    min_us: ordered[0],
    p50_us: percentile(0.50),
    p95_us: percentile(0.95),
    p99_us: percentile(0.99),
    max_us: ordered[ordered.length - 1]
  };
}

function runCollection(Constructor, add, remove) {
  const collection = new Constructor();
  const started = process.hrtime.bigint();
  for (let i = 0; i < iterations; i++) collection[add](i);
  for (let i = 0; i < iterations; i++) collection[remove]();
  return Number(process.hrtime.bigint() - started) / 1000;
}

function persistentProtocolStack() {
  const requests = [{ id: 'bench', op: 'create', kind: 'stack' }];
  for (let i = 0; i < protocolIterations; i++) {
    requests.push({ id: 'bench', op: 'call', method: 'push', args: [i] });
  }

  const input = `${requests.map(JSON.stringify).join('\n')}\n`;
  const started = process.hrtime.bigint();
  const result = spawnSync(rustDemo, [], {
    cwd: root,
    encoding: 'utf8',
    input,
    stdio: ['pipe', 'ignore', 'pipe']
  });
  if (result.status !== 0) {
    throw new Error(`persistent protocol runner exited with ${result.status}: ${result.stderr || ''}`);
  }
  return Number(process.hrtime.bigint() - started) / 1000;
}

function rustCore() {
  const output = execFileSync(rustBench, { cwd: root, encoding: 'utf8' });
  return JSON.parse(output.trim());
}

function startup(command, args) {
  const started = process.hrtime.bigint();
  const result = spawnSync(command, args, { cwd: root, stdio: 'ignore' });
  if (result.status !== 0) throw new Error(`${command} exited with ${result.status}`);
  return Number(process.hrtime.bigint() - started) / 1e6;
}

function startupSummary(command, args) {
  for (let i = 0; i < warmups; i++) startup(command, args);
  const samples = [];
  for (let i = 0; i < runs; i++) samples.push(startup(command, args) * 1000);
  const result = summary(samples);
  return {
    samples_ms: result.samples_us.map(sample => sample / 1000),
    min_ms: result.min_us / 1000,
    p50_ms: result.p50_us / 1000,
    p95_ms: result.p95_us / 1000,
    p99_ms: result.p99_us / 1000,
    max_ms: result.max_us / 1000
  };
}

function byteSummary(samples) {
  const ordered = samples.slice().sort((a, b) => a - b);
  const percentile = ratio => ordered[Math.min(ordered.length - 1, Math.ceil(ordered.length * ratio) - 1)];
  return {
    samples_bytes: samples,
    min_bytes: ordered[0],
    p50_bytes: percentile(0.50),
    p95_bytes: percentile(0.95),
    p99_bytes: percentile(0.99),
    max_bytes: ordered[ordered.length - 1]
  };
}

function rssSummary(kind, structure) {
  for (let i = 0; i < warmups; i++) rssSnapshot(kind, structure);
  const samples = [];
  for (let i = 0; i < runs; i++) samples.push(rssSnapshot(kind, structure).rss_bytes);
  return byteSummary(samples);
}

function rssSnapshot(kind, structure) {
  const output = execFileSync(process.execPath, [
    path.join('scripts', 'benchmark-rss-worker.js'),
    kind,
    structure,
    String(iterations)
  ], { cwd: root, encoding: 'utf8' });
  return JSON.parse(output.trim());
}

function rustRssSummary() {
  for (let i = 0; i < warmups; i++) rustRssSnapshot();
  const samples = [];
  for (let i = 0; i < runs; i++) samples.push(rustRssSnapshot().rss_bytes);
  return byteSummary(samples);
}

function rustRssSnapshot() {
  const output = execFileSync(process.execPath, [
    path.join('scripts', 'benchmark-rust-rss.js'),
    String(iterations)
  ], { cwd: root, encoding: 'utf8' });
  return JSON.parse(output.trim());
}

execFileSync('cargo', ['build', '--release', '--bin', 'bench', '--bin', 'rss_bench'], {
  cwd: root,
  stdio: 'inherit'
});

const rustRuns = [];
for (let i = 0; i < warmups; i++) rustCore();
for (let i = 0; i < runs; i++) rustRuns.push(rustCore());

const result = {
  generated_at: new Date().toISOString(),
  status: 'measured-local-distributions',
  environment: {
    os: process.platform,
    node: process.version,
    runs,
    warmups,
    iterations,
    protocol_iterations: protocolIterations,
    notes: 'Local Windows results. Rust core, N-API, upstream JS, and JSONL protocol use different runtime boundaries; compare distributions, not raw ratios.'
  },
  workloads: {
    stack_push_pop_200k: {
      rust_core: summary(rustRuns.map(run => run.stack_push_pop_us)),
      napi_adapter: measured(() => runCollection(NativeStack, 'push', 'pop')),
      original_node: measured(() => runCollection(OriginalStack, 'push', 'pop'))
    },
    queue_enqueue_dequeue_200k: {
      rust_core: summary(rustRuns.map(run => run.queue_enqueue_dequeue_us)),
      napi_adapter: measured(() => runCollection(NativeQueue, 'enqueue', 'dequeue')),
      original_node: measured(() => runCollection(OriginalQueue, 'enqueue', 'dequeue'))
    },
    persistent_jsonl_stack_push_10k: {
      rust_process: measured(persistentProtocolStack)
    }
  },
  startup: {
    rust_demo: startupSummary(rustDemo, []),
    original_node_module: startupSummary(process.execPath, ['-e', "require('./original/mnemonist/stack.js')"]),
    napi_node_module: startupSummary(process.execPath, ['-e', "require('./tests/stack.js')"])
  },
  rss: {
    methodology: 'Isolated Node child process RSS after the same 200k add/remove workload, plus Rust WorkingSet64/VmRSS while a Rust probe retains a 200k-item Stack and Queue. These are post-workload working sets, not OS peak RSS.',
    limitations: 'The Rust probe deliberately retains two filled collections while sampled, whereas Node samples occur after add/remove. Treat these as separately-defined working-set measurements, not a memory-efficiency ratio or peak RSS result.',
    rust_retained_stack_and_queue_200k: rustRssSummary(),
    stack_push_pop_200k: {
      napi_adapter: rssSummary('napi', 'stack'),
      original_node: rssSummary('original', 'stack')
    },
    queue_enqueue_dequeue_200k: {
      napi_adapter: rssSummary('napi', 'queue'),
      original_node: rssSummary('original', 'queue')
    }
  },
  unsafe_count: 0
};
const serialized = JSON.stringify(result, null, 2);

const outputPath = process.env.BENCH_OUTPUT || path.join('bench', 'results.json');
fs.writeFileSync(path.resolve(root, outputPath), `${serialized}\n`);

console.log(serialized);
