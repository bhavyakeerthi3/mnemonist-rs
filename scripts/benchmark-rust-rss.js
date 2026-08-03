'use strict';

const { execFile, spawn } = require('child_process');
const path = require('path');
const { promisify } = require('util');

const execFileAsync = promisify(execFile);
const root = path.resolve(__dirname, '..');
const executable = path.join(
  root,
  'target',
  'release',
  `rss_bench${process.platform === 'win32' ? '.exe' : ''}`
);
const items = Number(process.argv[2] || process.env.BENCH_RSS_ITEMS || 200000);

if (!Number.isInteger(items) || items <= 0) {
  throw new Error('usage: benchmark-rust-rss.js [positive-item-count]');
}

async function rssBytes(pid) {
  if (process.platform === 'win32') {
    const { stdout } = await execFileAsync('powershell.exe', [
      '-NoProfile',
      '-Command',
      `(Get-Process -Id ${pid}).WorkingSet64`
    ]);
    return Number(stdout.trim());
  }

  if (process.platform === 'linux') {
    const { stdout } = await execFileAsync('awk', [
      '/VmRSS:/ { print $2 * 1024; exit }',
      `/proc/${pid}/status`
    ]);
    return Number(stdout.trim());
  }

  throw new Error(`RSS sampling is not implemented for ${process.platform}`);
}

async function main() {
  const child = spawn(executable, [], {
    cwd: root,
    env: { ...process.env, MNEMONIST_RSS_ITEMS: String(items) },
    stdio: ['pipe', 'pipe', 'pipe']
  });
  let stdout = '';
  let stderr = '';

  child.stdout.setEncoding('utf8');
  child.stderr.setEncoding('utf8');
  child.stdout.on('data', chunk => { stdout += chunk; });
  child.stderr.on('data', chunk => { stderr += chunk; });

  await new Promise((resolve, reject) => {
    child.once('error', reject);
    child.stdout.on('data', () => {
      if (stdout.includes('ready ')) resolve();
    });
  });

  const rss = await rssBytes(child.pid);
  child.stdin.end('\n');
  const exitCode = await new Promise(resolve => child.once('close', resolve));
  if (exitCode !== 0) {
    throw new Error(`rss_bench exited with ${exitCode}: ${stderr}`);
  }

  console.log(JSON.stringify({
    rss_bytes: rss,
    items,
    method: process.platform === 'win32'
      ? 'Windows WorkingSet64 sampled while Rust retains stack and queue values'
      : 'Linux VmRSS sampled while Rust retains stack and queue values',
    stdout: stdout.trim()
  }));
}

main().catch(error => {
  console.error(error.stack || error.message);
  process.exitCode = 1;
});
