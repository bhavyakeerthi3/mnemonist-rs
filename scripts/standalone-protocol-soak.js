'use strict';

const assert = require('assert');
const { spawn } = require('child_process');
const path = require('path');

const args = new Map(process.argv.slice(2).map(argument => {
  const [key, value] = argument.split('=');
  return [key, value];
}));
const requests = Number(args.get('--requests') || 100000);
const root = path.resolve(__dirname, '..');
const executable = path.join(
  root,
  'target',
  'release',
  `mnemonist${process.platform === 'win32' ? '.exe' : ''}`
);

if (!Number.isInteger(requests) || requests < 8) {
  throw new Error('--requests must be an integer of at least 8.');
}

const cycles = Math.floor((requests - 4) / 4);
const protocol = [
  { id: 'soak-stack', op: 'create', kind: 'stack' },
  { id: 'soak-queue', op: 'create', kind: 'queue' }
];

for (let i = 0; i < cycles; i++) {
  protocol.push({ id: 'soak-stack', op: 'call', method: 'push', args: [i] });
  protocol.push({ id: 'soak-stack', op: 'call', method: 'pop', args: [] });
  protocol.push({ id: 'soak-queue', op: 'call', method: 'enqueue', args: [i] });
  protocol.push({ id: 'soak-queue', op: 'call', method: 'dequeue', args: [] });
}
protocol.push({ id: 'soak-stack', op: 'call', method: 'values', args: [] });
protocol.push({ id: 'soak-queue', op: 'call', method: 'values', args: [] });

const started = process.hrtime.bigint();
const child = spawn(executable, [], { cwd: root, stdio: ['pipe', 'pipe', 'pipe'] });
let stdout = '';
let stderr = '';
child.stdout.setEncoding('utf8');
child.stderr.setEncoding('utf8');
child.stdout.on('data', chunk => { stdout += chunk; });
child.stderr.on('data', chunk => { stderr += chunk; });
child.stdin.end(`${protocol.map(JSON.stringify).join('\n')}\n`);

child.once('error', error => { throw error; });
child.once('close', code => {
  if (code !== 0) throw new Error(`standalone protocol exited with ${code}: ${stderr}`);
  const responses = stdout.trim().split('\n').filter(Boolean).map(JSON.parse);
  assert.strictEqual(responses.length, protocol.length, 'response count');
  assert.deepStrictEqual(responses[0].result, { kind: 'void' }, 'stack create');
  assert.deepStrictEqual(responses[1].result, { kind: 'void' }, 'queue create');

  for (let i = 0; i < cycles; i++) {
    const offset = 2 + i * 4;
    assert.deepStrictEqual(responses[offset].result, { kind: 'value', value: 1 }, `stack push ${i}`);
    assert.deepStrictEqual(responses[offset + 1].result, { kind: 'value', value: i }, `stack pop ${i}`);
    assert.deepStrictEqual(responses[offset + 2].result, { kind: 'value', value: 1 }, `queue enqueue ${i}`);
    assert.deepStrictEqual(responses[offset + 3].result, { kind: 'value', value: i }, `queue dequeue ${i}`);
  }
  assert.deepStrictEqual(responses.at(-2).result, { kind: 'value', value: [] }, 'stack final state');
  assert.deepStrictEqual(responses.at(-1).result, { kind: 'value', value: [] }, 'queue final state');

  const elapsedMs = Number(process.hrtime.bigint() - started) / 1e6;
  console.log(JSON.stringify({
    status: 'pass',
    requests: protocol.length,
    cycles,
    elapsed_ms: elapsedMs,
    requests_per_second: protocol.length / (elapsedMs / 1000),
    transport: 'one persistent Rust JSONL process; every response checked'
  }, null, 2));
});
