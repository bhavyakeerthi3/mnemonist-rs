'use strict';

const assert = require('assert');

const args = new Map(process.argv.slice(2).map(argument => {
  const [key, value] = argument.split('=');
  return [key, value];
}));
const durationMs = Number(args.get('--duration-ms') || 60000);
const maxSteps = args.has('--steps') ? Number(args.get('--steps')) : Infinity;
let state = Number(args.get('--seed') || 0x51a5e11) >>> 0;

if ((!Number.isFinite(durationMs) || durationMs <= 0) || (args.has('--steps') && (!Number.isFinite(maxSteps) || maxSteps <= 0))) {
  throw new Error('--duration-ms and --steps must be positive numbers.');
}

process.env.MNEMONIST_TRANSPORT = 'protocol';
const RustSymSpell = require('../tests/symspell.js');
delete process.env.MNEMONIST_TRANSPORT;
const OriginalSymSpell = require('../original/mnemonist/symspell.js');

function random() {
  state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
  return state / 0x100000000;
}

function word() {
  const alphabet = 'abcdefhilmnoprstuwy';
  const length = Math.floor(random() * 9);
  let value = '';
  for (let i = 0; i < length; i++) value += alphabet[Math.floor(random() * alphabet.length)];
  return value;
}

function runCampaign(maxDistance, verbosity) {
  const rust = new RustSymSpell({ maxDistance, verbosity });
  const original = new OriginalSymSpell({ maxDistance, verbosity });
  const deadline = Date.now() + durationMs;
  const operations = [];
  let steps = 0;

  while (Date.now() < deadline && steps < maxSteps) {
    const roll = random();
    if (roll < 0.56) {
      const value = word();
      operations.push(['add', value]);
      rust.add(value);
      original.add(value);
    }
    else if (roll < 0.94) {
      const query = word();
      operations.push(['search', query]);
      try {
        assert.deepStrictEqual(rust.search(query), original.search(query));
      }
      catch (error) {
        throw new Error(
          `SymSpell(${maxDistance}, ${verbosity}) divergence at step ${steps}: ${error.message}\n` +
          `recent operations: ${operations.slice(-20).map(operation => `${operation[0]}(${JSON.stringify(operation[1])})`).join(', ')}\n` +
          `reproducer operations: ${JSON.stringify(operations)}`
        );
      }
    }
    else {
      operations.push(['clear']);
      rust.clear();
      original.clear();
    }

    assert.strictEqual(rust.size, original.size, `size divergence at step ${steps}`);
    steps++;
  }

  return steps;
}

const campaigns = [
  [1, 0],
  [2, 1],
  [2, 2],
  [4, 2]
];
const steps = campaigns.map(([maxDistance, verbosity]) => ({
  maxDistance,
  verbosity,
  steps: runCampaign(maxDistance, verbosity)
}));

console.log(`standalone SymSpell differential fuzz passed; seed=${state}`);
for (const campaign of steps) {
  console.log(`  maxDistance=${campaign.maxDistance} verbosity=${campaign.verbosity}: ${campaign.steps} synchronized operations`);
}
console.log(`  total synchronized operations: ${steps.reduce((total, campaign) => total + campaign.steps, 0)}`);
