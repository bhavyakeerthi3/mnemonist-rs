'use strict';

const childProcess = require('child_process');
const path = require('path');

const root = path.join(__dirname, '..');
const executable = path.join(
  root,
  'target',
  'release',
  process.platform === 'win32' ? 'mnemonist.exe' : 'mnemonist'
);

let nextId = 0;
let nextHandle = 0;
const objectHandles = new WeakMap();
const handles = new Map();
const weakHandles = new Map();
const tag = '__mnemonist_protocol_type__';

const weakMapFinalizer = new FinalizationRegistry(({ collection, handle }) => {
  const target = collection.deref();
  if (target) target._releaseWeakHandle(handle);
});

function encode(value) {
  if (value === undefined) return { [tag]: 'undefined' };
  if (value !== null && (typeof value === 'object' || typeof value === 'function')) {
    let handle = objectHandles.get(value);
    if (handle === undefined) {
      handle = nextHandle++;
      objectHandles.set(value, handle);
      handles.set(handle, value);
    }
    return { [tag]: 'handle', handle };
  }
  return value;
}

function decode(value) {
  if (Array.isArray(value)) return value.map(decode);
  if (!value || typeof value !== 'object') return value;
  if (value[tag] === 'undefined') return undefined;
  if (value[tag] === 'handle') {
    if (handles.has(value.handle)) return handles.get(value.handle);
    const weakValue = weakHandles.get(value.handle)?.deref();
    if (weakValue !== undefined) return weakValue;
    throw new Error(`mnemonist protocol returned unknown or collected handle: ${value.handle}`);
  }
  const decoded = {};
  for (const key of Object.keys(value)) decoded[key] = decode(value[key]);
  return decoded;
}

function resultValue(result) {
  if (result.kind === 'undefined' || result.kind === 'void') return undefined;
  if (result.kind === 'value') return decode(result.value);
  throw new Error(`mnemonist protocol returned an unknown result kind: ${result.kind}`);
}

class ProtocolCollection {
  constructor(kind, creationArgs) {
    this._id = `${kind}-${nextId++}`;
    this._kind = kind;
    this._history = [];
    this._size = 0;
    this._creationArgs = creationArgs;
    this._weakKeyHandles = new WeakMap();
    this._weakValues = new WeakMap();
  }

  call(method, args) {
    const request = { id: this._id, op: 'call', method, args: args.map(encodeData) };
    const response = this._run([...this._history, request]);
    this._history.push(request);
    this._size = response.size;
    return resultValue(response.result);
  }

  callOpaque(method, args) {
    const request = { id: this._id, op: 'call', method, args: args.map(encode) };
    const response = this._run([...this._history, request]);
    this._history.push(request);
    this._size = response.size;
    return resultValue(response.result);
  }

  callWeakMap(method, args) {
    let encodedArgs;

    if (method === 'clear') {
      this._weakValues = new WeakMap();
      encodedArgs = [];
    } else {
      const key = args[0];
      if (key === null || (typeof key !== 'object' && typeof key !== 'function')) {
        throw new TypeError('mnemonist/DefaultWeakMap: keys must be objects or functions.');
      }

      const encodedKey = this._encodeWeakKey(key);
      if (method === 'set') this._weakValues.set(key, args[1]);
      if (method === 'delete') this._weakValues.delete(key);
      encodedArgs = method === 'set' ? [encodedKey, this._encodeWeakValue(args[1])] : [encodedKey];
    }

    const request = { id: this._id, op: 'call', method, args: encodedArgs };
    const response = this._run([...this._history, request]);
    this._history.push(request);
    this._size = response.size;
    return resultValue(response.result);
  }

  size() {
    return this._size;
  }

  _run(requests) {
    const input = [
      { id: this._id, op: 'create', kind: this._kind, args: this._creationArgs.map(encodeData) },
      ...requests
    ].map(JSON.stringify).join('\n') + '\n';

    const run = childProcess.spawnSync(executable, [], {
      cwd: root,
      encoding: 'utf8',
      input
    });

    if (run.error) throw run.error;
    if (run.status !== 0) {
      throw new Error(`mnemonist protocol runner failed: ${run.stderr || run.status}`);
    }

    const responses = run.stdout.trim().split(/\r?\n/).map(JSON.parse);
    const response = responses[responses.length - 1];
    if (!response || !response.ok) {
      throw new Error(`mnemonist protocol error: ${response && response.error || 'no response'}`);
    }
    return response;
  }

  _encodeWeakKey(key) {
    let handle = this._weakKeyHandles.get(key);
    if (handle === undefined) {
      handle = objectHandles.get(key);
      if (handle === undefined) {
        handle = nextHandle++;
        objectHandles.set(key, handle);
      }
      this._weakKeyHandles.set(key, handle);
      weakHandles.set(handle, new WeakRef(key));
      weakMapFinalizer.register(key, { collection: new WeakRef(this), handle });
    }
    return { [tag]: 'handle', handle };
  }

  _encodeWeakValue(value) {
    if (value === undefined) return { [tag]: 'undefined' };
    if (value !== null && (typeof value === 'object' || typeof value === 'function')) {
      let handle = objectHandles.get(value);
      if (handle === undefined) {
        handle = nextHandle++;
        objectHandles.set(value, handle);
      }
      weakHandles.set(handle, new WeakRef(value));
      return { [tag]: 'handle', handle };
    }
    return value;
  }

  _releaseWeakHandle(handle) {
    this._history.push({
      id: this._id,
      op: 'call',
      method: 'release',
      args: [{ [tag]: 'handle', handle }]
    });
  }
}

function create(kind, creationArgs = []) {
  return new ProtocolCollection(kind, creationArgs);
}

function encodeData(value) {
  if (Array.isArray(value)) return value.map(encodeData);
  return encode(value);
}

function invoke(kind, method, args = []) {
  const collection = new ProtocolCollection(kind, []);
  const response = collection._run([{
    id: collection._id,
    op: 'call',
    method,
    args: args.map(encodeData)
  }]);
  return resultValue(response.result);
}

module.exports = { create, invoke };
