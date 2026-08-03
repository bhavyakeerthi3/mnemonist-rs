/**
 * Shared helpers for mnemonist JS adapter shims (Port Mortem).
 */
// Protocol mode deliberately avoids loading the N-API addon. It is used only
// by the standalone conformance subset; the default adapter path remains N-API.
const native = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? null
  : require('../index.js');

function iteratorFrom(valuesFn) {
  const items = valuesFn();
  let i = 0;
  return {
    next() {
      if (i >= items.length) return { done: true };
      return { value: items[i++], done: false };
    },
    [Symbol.iterator]() {
      return this;
    }
  };
}

function entriesIterator(valuesFn) {
  const items = valuesFn();
  let i = 0;
  return {
    next() {
      if (i >= items.length) return { done: true };
      return { value: [i, items[i++]], done: false };
    },
    [Symbol.iterator]() {
      return this;
    }
  };
}

function addIterableMethods(proto, valuesFn) {
  proto.forEach = function(callback, scope) {
    scope = arguments.length > 1 ? scope : this;
    const items = valuesFn.call(this);
    for (let i = 0; i < items.length; i++) {
      callback.call(scope, items[i], i, this);
    }
  };

  proto.values = function() {
    return iteratorFrom(() => valuesFn.call(this));
  };

  proto.entries = function() {
    return entriesIterator(() => valuesFn.call(this));
  };

  proto[Symbol.iterator] = proto.values;
};

/**
 * Rust's `Option<T>::None` crosses the N-API boundary as JS `null`, but the
 * original mnemonist API returns `undefined` for "nothing here" results
 * (empty peek/pop/shift/etc). Normalize at the adapter boundary so behavior
 * matches upstream exactly, while keeping the Rust side's `Option` idiomatic.
 */
function nullToUndefined(value) {
  return value === null ? undefined : value;
}

module.exports = {
  native,
  addIterableMethods,
  nullToUndefined
};
