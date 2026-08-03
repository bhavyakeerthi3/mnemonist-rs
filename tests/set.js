const { native } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol' ? require('../adapter/_protocol.js') : null;

function toSet(values) {
  return new Set(values);
}

function mutateFromVectors(mutator, a, b) {
  const setA = new Set(a);
  mutator(setA, new Set(b));
  return setA;
}

exports.intersection = function() {
  if (arguments.length < 2) {
    throw new Error('mnemonist/Set.intersection: needs at least two arguments.');
  }
  const sets = Array.from(arguments).map(s => Array.from(s));
  return toSet(protocol ? protocol.invoke('set-ops', 'intersection', [sets]) : native.setIntersection(sets));
};

exports.union = function() {
  if (arguments.length < 2) {
    throw new Error('mnemonist/Set.union: needs at least two arguments.');
  }
  const sets = Array.from(arguments).map(s => Array.from(s));
  return toSet(protocol ? protocol.invoke('set-ops', 'union', [sets]) : native.setUnion(sets));
};

exports.difference = function(A, B) {
  return toSet(protocol ? protocol.invoke('set-ops', 'difference', [Array.from(A), Array.from(B)]) : native.setDifference(Array.from(A), Array.from(B)));
};

exports.symmetricDifference = function(A, B) {
  return toSet(protocol ? protocol.invoke('set-ops', 'symmetricDifference', [Array.from(A), Array.from(B)]) : native.setSymmetricDifference(Array.from(A), Array.from(B)));
};

exports.isSubset = function(A, B) {
  return protocol ? protocol.invoke('set-ops', 'isSubset', [Array.from(A), Array.from(B)]) : native.setIsSubset(Array.from(A), Array.from(B));
};

exports.isSuperset = function(A, B) {
  return protocol ? protocol.invoke('set-ops', 'isSuperset', [Array.from(A), Array.from(B)]) : native.setIsSuperset(Array.from(A), Array.from(B));
};

exports.add = function(A, B) {
  for (const item of B) A.add(item);
};

exports.subtract = function(A, B) {
  for (const item of B) A.delete(item);
};

exports.intersect = function(A, B) {
  for (const item of Array.from(A)) {
    if (!B.has(item)) A.delete(item);
  }
};

exports.disjunct = function(A, B) {
  const toRemove = [];
  for (const item of A) {
    if (B.has(item)) toRemove.push(item);
  }
  for (const item of B) {
    if (!A.has(item)) A.add(item);
  }
  for (const item of toRemove) A.delete(item);
};

exports.intersectionSize = function(A, B) {
  return protocol ? protocol.invoke('set-ops', 'intersectionSize', [Array.from(A), Array.from(B)]) : native.setIntersectionSize(Array.from(A), Array.from(B));
};

exports.unionSize = function(A, B) {
  return protocol ? protocol.invoke('set-ops', 'unionSize', [Array.from(A), Array.from(B)]) : native.setUnionSize(Array.from(A), Array.from(B));
};

exports.jaccard = function(A, B) {
  return protocol ? protocol.invoke('set-ops', 'jaccard', [Array.from(A), Array.from(B)]) : native.setJaccard(Array.from(A), Array.from(B));
};

exports.overlap = function(A, B) {
  return protocol ? protocol.invoke('set-ops', 'overlap', [Array.from(A), Array.from(B)]) : native.setOverlap(Array.from(A), Array.from(B));
};
