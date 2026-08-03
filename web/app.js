'use strict';

const modules = {
  stack: module('STACK', 'stack', 'CLASSICS', [], 'VALUE JSON', null, '"neon"', '"arcade"', actions('PUSH push primary', 'POP pop none', 'PEEK peek none view', 'CLEAR clear none')),
  queue: module('QUEUE', 'queue', 'CLASSICS', [], 'VALUE JSON', null, '"player-one"', '"arcade"', actions('ENQUEUE enqueue primary', 'DEQUEUE dequeue none', 'PEEK peek none view', 'CLEAR clear none')),
  'linked-list': module('LINKED LIST', 'linked-list', 'CLASSICS', [], 'VALUE JSON', null, '"node"', '"first"', actions('PUSH push primary', 'UNSHIFT unshift primary', 'SHIFT shift none', 'LAST last none view')),
  'lru-cache': module('LRU CACHE', 'lru-cache', 'CLASSICS', [4], 'VALUE JSON', 'KEY JSON', '"checkpoint"', '"save-data"', actions('SET set secondary-primary', 'GET get secondary view', 'PEEK peek secondary view', 'CLEAR clear none'), '"slot-a"'),
  'multi-map': module('MULTI MAP', 'multi-map', 'CLASSICS', [false], 'VALUE JSON', 'KEY JSON', '"green"', '"team"', actions('SET set secondary-primary', 'GET get secondary view', 'REMOVE remove secondary-primary', 'CLEAR clear none'), '"arcade"'),
  'multi-set': module('MULTI SET', 'multi-set', 'CLASSICS', [], 'VALUE JSON', 'COUNT', '"token"', '"power-up"', actions('ADD add pair', 'HAS has primary view', 'REMOVE remove pair', 'CLEAR clear none'), '1'),
  heap: module('HEAP', 'heap', 'CLASSICS', [false], 'VALUE JSON', null, '42', '7', actions('PUSH push primary', 'POP pop none', 'PEEK peek none view', 'CLEAR clear none')),
  'trie-map': module('TRIE MAP', 'trie-map', 'CLASSICS', [], 'VALUE JSON', 'KEY JSON', '"value"', '"arcade"', actions('SET set secondary-primary', 'GET get secondary view', 'DELETE delete secondary', 'CLEAR clear none'), '"mnemo"'),
  'bi-map': module('BI MAP', 'bi-map', 'CLASSICS', [], 'VALUE JSON', 'KEY JSON', '"rust"', '"language"', actions('SET set secondary-primary', 'GET get secondary view', 'INVERSE_GET inverseGet primary view', 'DELETE delete secondary'), '"js"'),
  'set-ops': module('SET HELPERS', 'set-ops', 'CLASSICS', [], 'LEFT ARRAY', 'RIGHT ARRAY', '[1,2,3]', '[2,3,4]', actions('UNION union pair view', 'INTERSECTION intersection pair view', 'DIFFERENCE difference pair view', 'SYMMETRIC symmetricDifference pair view'), '[3,4,5]'),

  'fixed-stack': module('FIXED STACK', 'fixed-stack', 'LOW LEVEL', [8], 'VALUE JSON', null, '"frame"', '"slot"', actions('PUSH push primary', 'POP pop none', 'PEEK peek none view', 'CLEAR clear none')),
  'fixed-deque': module('FIXED DEQUE', 'fixed-deque', 'LOW LEVEL', [8], 'VALUE JSON', null, '"frame"', '"front"', actions('PUSH push primary', 'UNSHIFT unshift primary', 'POP pop none', 'SHIFT shift none')),
  'circular-buffer': module('CIRCULAR BUFFER', 'circular-buffer', 'LOW LEVEL', [8], 'VALUE JSON', null, '"frame"', '"loop"', actions('PUSH push primary', 'UNSHIFT unshift primary', 'POP pop none', 'SHIFT shift none')),
  'sparse-set': module('SPARSE SET', 'sparse-set', 'LOW LEVEL', [32], 'INDEX', null, '7', '4', actions('ADD add primary', 'HAS has primary view', 'DELETE delete primary', 'CLEAR clear none')),
  'sparse-queue-set': module('SPARSE QUEUE SET', 'sparse-queue-set', 'LOW LEVEL', [32], 'INDEX', null, '7', '4', actions('ENQUEUE enqueue primary', 'DEQUEUE dequeue none', 'HAS has primary view', 'CLEAR clear none')),
  'sparse-map': module('SPARSE MAP', 'sparse-map', 'LOW LEVEL', [32], 'VALUE JSON', 'INDEX', '"value"', '"slot"', actions('SET set secondary-primary', 'GET get secondary view', 'DELETE delete secondary', 'CLEAR clear none'), '7'),
  'hashed-array-tree': module('HASHED ARRAY TREE', 'hashed-array-tree', 'LOW LEVEL', [0, 0, 64], 'VALUE JSON', 'INDEX', '"node"', '"tree"', actions('PUSH push primary', 'GET get secondary view', 'SET set secondary-primary', 'CLEAR clear none'), '0'),
  vector: module('VECTOR', 'vector', 'LOW LEVEL', [8, 0], 'VALUE JSON', 'INDEX', '"item"', '"vector"', actions('PUSH push primary', 'GET get secondary view', 'SET set secondary-primary', 'CLEAR clear none'), '0'),
  'bit-set': module('BIT SET', 'bit-set', 'LOW LEVEL', [64], 'INDEX', 'BIT', '4', '7', actions('SET set pair', 'TEST test primary view', 'FLIP flip primary', 'RESET reset primary'), 'true'),
  'bit-vector': module('BIT VECTOR', 'bit-vector', 'LOW LEVEL', [0], 'BIT', null, 'true', 'false', actions('PUSH push primary', 'POP pop none', 'VALUES values none view', 'CLEAR clear none')),
  'static-disjoint-set': module('STATIC DISJOINT SET', 'static-disjoint-set', 'LOW LEVEL', [8], 'LEFT INDEX', 'RIGHT INDEX', '0', '0', actions('UNION union pair', 'CONNECTED connected pair view', 'COMPILE compile none view', 'MAPPING mapping none view'), '1'),
  'multi-array': module('MULTI ARRAY', 'multi-array', 'LOW LEVEL', [8], 'VALUE JSON', 'INDEX', '"sprite"', '"array"', actions('PUSH push primary', 'SET set secondary-primary', 'GET get secondary view', 'CLEAR clear none'), '0'),

  'fuzzy-map': module('FUZZY MAP', 'fuzzy-map', 'SEARCH & NLP', [], 'VALUE JSON', 'KEY JSON', '"result"', '"arcade"', actions('SET set secondary-primary', 'GET get secondary view', 'HAS has secondary view', 'CLEAR clear none'), '"mnemo"'),
  'fuzzy-multi-map': module('FUZZY MULTI MAP', 'fuzzy-multi-map', 'SEARCH & NLP', [false], 'VALUE JSON', 'KEY JSON', '"result"', '"arcade"', actions('SET set secondary-primary', 'GET get secondary view', 'CLEAR clear none')),
  'inverted-index': module('INVERTED INDEX', 'inverted-index', 'SEARCH & NLP', [], 'DOCUMENT JSON', 'TOKENS JSON', '"doc-1"', '"welcome"', actions('ADD add pair', 'GET get secondary view', 'CLEAR clear none'), '["rust","arcade"]'),
  symspell: module('SYMSPELL', 'symspell', 'SEARCH & NLP', [2, 2], 'WORD', null, 'mnemonist', 'arcade', actions('ADD add primary', 'SEARCH search primary view', 'SIZE size none view', 'CLEAR clear none')),
  'bk-tree': module('BK TREE', 'bk-tree', 'SEARCH & NLP', [], 'WORD', null, 'arcade', 'rust', actions('ADD add primary', 'SEARCH search primary view', 'SIZE size none view', 'CLEAR clear none')),
  'passjoin-index': module('PASSJOIN INDEX', 'passjoin-index', 'SEARCH & NLP', [2], 'STRING', null, 'arcade', 'arcadia', actions('ADD add primary', 'SEARCH search primary view', 'CLEAR clear none')),

  'suffix-array': module('SUFFIX ARRAY', 'suffix-array', 'INDEXATION', ['banana'], 'READ ONLY', null, 'banana', 'suffixes', actions('RELOAD reset none'), '""'),
  'generalized-suffix-array': module('GENERALIZED SUFFIX ARRAY', 'generalized-suffix-array', 'INDEXATION', [['banana', 'bandana']], 'READ ONLY', null, '["banana","bandana"]', 'suffixes', actions('RELOAD reset none')),
  'static-interval-tree': module('STATIC INTERVAL TREE', 'static-interval-tree', 'INDEXATION', [[[0, 5], [4, 10], [9, 12]]], 'READ ONLY', null, '[[0,5],[4,10]]', 'intervals', actions('RELOAD reset none')),
  'kd-tree': module('KD TREE', 'kd-tree', 'INDEXATION', [['alpha', 'beta', 'gamma'], [[0, 0], [3, 4], [1, 1]]], 'QUERY JSON', null, '[1,1]', 'nearest', actions('NEAREST nearest primary view', 'RELOAD reset none')),
  'vp-tree': module('VP TREE', 'vp-tree', 'INDEXATION', [['arcade', 'arcadia', 'rust']], 'QUERY', null, 'arcade', 'nearest', actions('NEAREST nearest primary view', 'RELOAD reset none')),
  'critbit-tree-map': module('CRITBIT TREE MAP', 'critbit-tree-map', 'INDEXATION', [], 'VALUE JSON', 'KEY STRING', '"value"', '"critbit"', actions('SET set secondary-primary', 'GET get secondary view', 'DELETE delete secondary', 'CLEAR clear none'), 'arcade'),
  'fixed-critbit-tree-map': module('FIXED CRITBIT TREE MAP', 'fixed-critbit-tree-map', 'INDEXATION', [8], 'VALUE JSON', 'KEY STRING', '"value"', '"fixed"', actions('SET set secondary-primary', 'GET get secondary view', 'DELETE delete secondary', 'CLEAR clear none'), 'arcade'),

  'bloom-filter': module('BLOOM FILTER', 'bloom-filter', 'PROBABILISTIC', [100, 0.01], 'VALUE JSON', null, '"rust"', '"mnemonist"', actions('ADD add primary', 'TEST test primary view', 'CLEAR clear none')),
  'default-map': module('DEFAULT MAP', 'default-map', 'UTILITY', [], 'VALUE JSON', 'KEY JSON', '"value"', '"default"', actions('SET set secondary-primary', 'GET get secondary view', 'DELETE delete secondary', 'CLEAR clear none'), '"arcade"'),
  'default-weak-map': module('DEFAULT WEAK MAP', 'default-weak-map', 'UTILITY', [], 'VALUE JSON', 'KEY JSON', '"value"', '"weak"', actions('SET set secondary-primary', 'GET get secondary view', 'DELETE delete secondary', 'CLEAR clear none'), '"arcade"'),
  'fixed-reverse-heap': module('FIXED REVERSE HEAP', 'fixed-reverse-heap', 'UTILITY', [8], 'VALUE JSON', null, '42', '7', actions('PUSH push primary', 'POP pop none', 'PEEK peek none view', 'CLEAR clear none')),
  'comparator-heap': module('COMPARATOR HEAP', 'comparator-heap', 'UTILITY', [false], 'VALUE JSON', 'COMPARATOR JSON', '42', '7', actions('PUSH push primary', 'POP pop secondary view', 'CLEAR clear none')),
  sort: module('SORT HELPERS', 'sort', 'UTILITY', [], 'VALUES JSON', null, '[3,1,2]', '[5,2,4]', actions('SORT sort primary view', 'INSERTION_SORT insertionSort primary view', 'RELOAD reset none'))
};

function module(title, kind, category, args, primary, secondary, placeholder, sample, actions, secondaryValue = '') {
  return { title, kind, category, args, primary, secondary, placeholder, sample, actions, secondaryValue };
}

function actions(...descriptions) {
  return descriptions.map(description => {
    const [label, method, argMode = 'none', output = 'state'] = description.split(' ');
    return [label.replaceAll('_', ' '), method, argMode, output];
  });
}

const elements = {
  list: document.querySelector('#collection-list'),
  search: document.querySelector('#module-search'),
  count: document.querySelector('#module-count'),
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
  engine: document.querySelector('#engine-value'),
  launchGate: document.querySelector('#launch-gate'),
  enterArcade: document.querySelector('#enter-arcade')
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
  await protocol(operationHistory);
  await snapshot();
}

async function snapshot() {
  const response = await protocol([...operationHistory, { id: session, op: 'snapshot' }]);
  const rawValues = response.result?.value?.values;
  const values = Array.isArray(rawValues) ? rawValues : rawValues == null ? [] : [rawValues];
  renderState(values, response.size || 0);
}

async function action(method, argMode = 'none', output = 'state') {
  const definition = modules[active];
  const primary = parseValue(elements.primary.value);
  const secondary = parseValue(elements.secondary.value);
  const args = argumentsFor(argMode, primary, secondary);

  try {
    if (method === 'reset') {
      await reset();
      return;
    }
    operationHistory.push({ id: session, op: 'call', method, args });
    const response = await protocol(operationHistory);
    if (output === 'view') renderState([response.result?.value ?? response.result], response.size || 0);
    else await snapshot();
  } catch (error) {
    trace('ERROR', { message: error.message, module: definition.title, method });
    elements.empty.textContent = `ERROR: ${error.message.toUpperCase()}`;
    elements.empty.classList.remove('is-hidden');
  }
}

function argumentsFor(mode, primary, secondary) {
  if (mode === 'primary') return [primary];
  if (mode === 'secondary') return [secondary];
  if (mode === 'pair') return [primary, secondary];
  if (mode === 'secondary-primary') return [secondary, primary];
  return [];
}

function tileValue(value) {
  if (value && typeof value === 'object') return JSON.stringify(value);
  return String(value);
}

function renderState(values, size) {
  const previewLimit = 32;
  elements.size.textContent = `SIZE ${size}`;
  elements.tiles.innerHTML = '';
  if (!values.length) {
    elements.empty.textContent = 'READY FOR INPUT';
    elements.empty.classList.remove('is-hidden');
    return;
  }
  elements.empty.classList.add('is-hidden');
  values.slice(0, previewLimit).forEach((value, index) => {
    const tile = document.createElement('div');
    tile.className = 'state-tile';
    tile.title = tileValue(value);
    tile.textContent = `${String(index + 1).padStart(2, '0')} ${tileValue(value)}`;
    elements.tiles.appendChild(tile);
  });
  if (values.length > previewLimit) {
    const summary = document.createElement('div');
    summary.className = 'state-tile state-summary';
    summary.textContent = `+ ${values.length - previewLimit} MORE IN TRACE`;
    elements.tiles.appendChild(summary);
  }
}

function chooseModule(name) {
  active = name;
  const definition = modules[active];
  renderSelector(elements.search.value);
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
  definition.actions.forEach(([label, method, argMode, output]) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.textContent = label;
    button.addEventListener('click', () => action(method, argMode, output));
    elements.actionGrid.appendChild(button);
  });
  reset().catch(error => trace('STARTUP ERROR', { message: error.message }));
}

function renderSelector(filter = '') {
  const query = filter.trim().toLowerCase();
  const visible = Object.entries(modules).filter(([, definition]) => (
    !query || definition.title.toLowerCase().includes(query) || definition.category.toLowerCase().includes(query)
  ));
  const groups = new Map();
  visible.forEach(([name, definition]) => {
    if (!groups.has(definition.category)) groups.set(definition.category, []);
    groups.get(definition.category).push([name, definition]);
  });
  elements.list.innerHTML = '';
  let number = 0;
  groups.forEach((entries, category) => {
    const heading = document.createElement('p');
    heading.className = 'collection-group';
    heading.textContent = category;
    elements.list.appendChild(heading);
    entries.forEach(([name, definition]) => {
      number += 1;
      const button = document.createElement('button');
      button.type = 'button';
      button.className = `collection-button${name === active ? ' is-active' : ''}`;
      button.dataset.collection = name;
      button.innerHTML = `<span>${String(number).padStart(2, '0')}</span>${definition.title}`;
      button.addEventListener('click', () => chooseModule(name));
      elements.list.appendChild(button);
    });
  });
  elements.count.textContent = `${visible.length} / ${Object.keys(modules).length} RUST MODULES`;

  if (!visible.length) {
    const empty = document.createElement('p');
    empty.className = 'selector-empty';
    empty.textContent = 'NO RUST MODULES MATCH';
    elements.list.appendChild(empty);
  }
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

elements.search.addEventListener('input', () => renderSelector(elements.search.value));
document.querySelector('#reset-button').addEventListener('click', () => reset().catch(error => trace('RESET ERROR', { message: error.message })));
document.querySelector('#sample-button').addEventListener('click', () => {
  elements.primary.value = modules[active].sample;
  if (active === 'lru') elements.secondary.value = '"checkpoint"';
});
document.querySelector('#clear-trace').addEventListener('click', () => { elements.trace.textContent = ''; });
elements.enterArcade.addEventListener('click', () => {
  elements.launchGate.classList.add('is-hidden');
  elements.launchGate.setAttribute('aria-hidden', 'true');
  elements.search.focus();
});

renderSelector();
boot();
