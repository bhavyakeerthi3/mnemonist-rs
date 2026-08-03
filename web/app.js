'use strict';

const modules = {
  stack: {
    title: 'STACK', kind: 'stack', args: [], primary: 'VALUE JSON', secondary: null,
    placeholder: '"neon"', sample: '"arcade"', actions: [
      ['PUSH', 'push'], ['POP', 'pop'], ['PEEK', 'peek'], ['CLEAR', 'clear']
    ]
  },
  queue: {
    title: 'QUEUE', kind: 'queue', args: [], primary: 'VALUE JSON', secondary: null,
    placeholder: '42', sample: '"player-one"', actions: [
      ['ENQUEUE', 'enqueue'], ['DEQUEUE', 'dequeue'], ['PEEK', 'peek'], ['CLEAR', 'clear']
    ]
  },
  lru: {
    title: 'LRU CACHE', kind: 'lru-cache', args: [4], primary: 'VALUE JSON', secondary: 'KEY JSON',
    placeholder: '"save-data"', secondaryValue: '"slot-a"', sample: '"checkpoint"', actions: [
      ['SET', 'set'], ['GET', 'get'], ['PEEK', 'peek'], ['CLEAR', 'clear']
    ]
  },
  'bit-vector': {
    title: 'BIT VECTOR', kind: 'bit-vector', args: [], primary: 'BIT', secondary: null,
    placeholder: 'true', sample: 'false', actions: [
      ['PUSH BIT', 'push'], ['POP BIT', 'pop'], ['VALUES', 'values'], ['CLEAR', 'clear']
    ]
  },
  symspell: {
    title: 'SYMSPELL', kind: 'symspell', args: [2, 2], primary: 'WORD', secondary: null,
    placeholder: 'mnemonist', sample: 'arcade', actions: [
      ['ADD', 'add'], ['SEARCH', 'search'], ['SIZE', 'size'], ['CLEAR', 'clear']
    ]
  }
};

const elements = {
  buttons: [...document.querySelectorAll('.collection-button')],
  title: document.querySelector('#module-title'),
  primaryLabel: document.querySelector('#primary-label'),
  primary: document.querySelector('#primary-input'),
  secondaryLabel: document.querySelector('#secondary-label'),
  secondary: document.querySelector('#secondary-input'),
  actionGrid: document.querySelector('#action-grid'),
  tiles: document.querySelector('#state-tiles'),
  empty: document.querySelector('#empty-state'),
  size: document.querySelector('#collection-size'),
  trace: document.querySelector('#protocol-trace'),
  status: document.querySelector('#runtime-status'),
  statusBox: document.querySelector('.runtime-status'),
  engine: document.querySelector('#engine-value')
};

let active = 'stack';
let session = '';
let requestNumber = 0;
let operationHistory = [];

function newSession() {
  session = `arcade-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`;
  requestNumber = 0;
}

function parseValue(value) {
  const trimmed = value.trim();
  if (!trimmed) return '';
  try { return JSON.parse(trimmed); } catch { return trimmed; }
}

function requestId() {
  requestNumber += 1;
  return `${session}-${requestNumber}`;
}

function trace(label, value) {
  const entry = `${label}\n${JSON.stringify(value, null, 2)}`;
  elements.trace.textContent = `${entry}\n\n${elements.trace.textContent}`.slice(0, 5000);
}

async function protocol(requests) {
  const payload = { requests };
  trace('REQUEST', payload);
  const response = await fetch('/api/protocol', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload)
  });
  const body = await response.json();
  trace('RUST RESPONSE', body);
  if (!response.ok || !body.ok) throw new Error(body.error || `HTTP ${response.status}`);
  return body;
}

async function reset() {
  const definition = modules[active];
  newSession();
  operationHistory = [{ id: session, op: 'create', kind: definition.kind, args: definition.args }];
  elements.trace.textContent = '';
  const response = await protocol(operationHistory);
  renderState([], response.size || 0);
}

async function snapshot() {
  const response = await protocol([...operationHistory, { id: session, op: 'snapshot' }]);
  const rawValues = response.result?.value?.values;
  const values = Array.isArray(rawValues) ? rawValues : rawValues == null ? [] : [rawValues];
  renderState(values, response.size || 0);
}

async function action(method) {
  const definition = modules[active];
  let args = [];
  const primary = parseValue(elements.primary.value);
  const secondary = parseValue(elements.secondary.value);

  if (['push', 'enqueue', 'add', 'search'].includes(method)) args = [primary];
  if (method === 'set') args = [secondary, primary];
  if (['get', 'peek'].includes(method) && active === 'lru') args = [secondary];
  if (method === 'clear') args = [];

  try {
    operationHistory.push({ id: session, op: 'call', method, args });
    const response = await protocol(operationHistory);
    if (method === 'add' && active === 'symspell') {
      renderState([`INDEXED: ${String(primary)}`], response.size || 0);
    } else if (method !== 'search' && method !== 'size') {
      await snapshot();
    } else {
      renderState([response.result], response.size || 0);
    }
  } catch (error) {
    trace('ERROR', { message: error.message, module: definition.title, method });
    elements.empty.textContent = `ERROR: ${error.message.toUpperCase()}`;
    elements.empty.classList.remove('is-hidden');
  }
}

function tileValue(value) {
  if (value && typeof value === 'object') return JSON.stringify(value);
  return String(value);
}

function renderState(values, size) {
  elements.size.textContent = `SIZE ${size}`;
  elements.tiles.innerHTML = '';
  if (!values.length) {
    elements.empty.textContent = 'READY FOR INPUT';
    elements.empty.classList.remove('is-hidden');
    return;
  }
  elements.empty.classList.add('is-hidden');
  values.forEach((value, index) => {
    const tile = document.createElement('div');
    tile.className = 'state-tile';
    tile.title = tileValue(value);
    tile.textContent = `${String(index + 1).padStart(2, '0')} ${tileValue(value)}`;
    elements.tiles.appendChild(tile);
  });
}

function chooseModule(name) {
  active = name;
  const definition = modules[active];
  elements.buttons.forEach(button => button.classList.toggle('is-active', button.dataset.collection === active));
  elements.title.textContent = definition.title;
  elements.primaryLabel.textContent = definition.primary;
  elements.primary.placeholder = definition.placeholder;
  elements.primary.value = definition.placeholder;
  const showSecondary = Boolean(definition.secondary);
  elements.secondaryLabel.classList.toggle('secondary-field', !showSecondary);
  elements.secondary.classList.toggle('secondary-field', !showSecondary);
  elements.secondaryLabel.style.display = showSecondary ? '' : 'none';
  elements.secondary.style.display = showSecondary ? '' : 'none';
  elements.secondaryLabel.textContent = definition.secondary || '';
  elements.secondary.value = definition.secondaryValue || '';
  elements.actionGrid.innerHTML = '';
  definition.actions.forEach(([label, method]) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.textContent = label;
    button.addEventListener('click', () => action(method));
    elements.actionGrid.appendChild(button);
  });
  reset().catch(error => trace('STARTUP ERROR', { message: error.message }));
}

async function boot() {
  try {
    const health = await fetch('/api/health').then(response => response.json());
    elements.status.textContent = 'ONLINE';
    elements.statusBox.classList.add('is-online');
    elements.engine.textContent = health.engine;
  } catch (error) {
    elements.status.textContent = 'OFFLINE';
    elements.engine.textContent = 'Rust service unavailable';
    trace('HEALTH ERROR', { message: error.message });
  }
  chooseModule(active);
}

elements.buttons.forEach(button => button.addEventListener('click', () => chooseModule(button.dataset.collection)));
document.querySelector('#reset-button').addEventListener('click', () => reset().catch(error => trace('RESET ERROR', { message: error.message })));
document.querySelector('#sample-button').addEventListener('click', () => {
  elements.primary.value = modules[active].sample;
  if (active === 'lru') elements.secondary.value = '"checkpoint"';
});
document.querySelector('#clear-trace').addEventListener('click', () => { elements.trace.textContent = ''; });

boot();
