'use strict';

const standardLevenshtein = require('leven');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function pointerType(size) {
  return size <= 0xff ? Uint8Array : size <= 0xffff ? Uint16Array : Uint32Array;
}

function quickSortIndices(values, indices, lo, hi) {
  const los = [lo];
  const his = [hi];
  let level = 0;
  while (level >= 0) {
    let left = los[level];
    let right = his[level] - 1;
    if (left < right) {
      const item = indices[left];
      const pivot = values[item];
      while (left < right) {
        while (values[indices[right]] >= pivot && left < right) right--;
        if (left < right) indices[left++] = indices[right];
        while (values[indices[left]] <= pivot && left < right) left++;
        if (left < right) indices[right--] = indices[left];
      }
      indices[left] = item;
      los[level + 1] = left + 1;
      his[level + 1] = his[level];
      his[level++] = left;
      if (his[level] - los[level] > his[level - 1] - los[level - 1]) {
        [los[level], los[level - 1]] = [los[level - 1], los[level]];
        [his[level], his[level - 1]] = [his[level - 1], his[level]];
      }
    }
    else {
      level--;
    }
  }
}

function lowerBound(values, indices, value, lo, hi) {
  while (lo < hi) {
    const mid = (lo + hi) >>> 1;
    if (value <= values[indices[mid]]) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}

function createBinaryTree(distance, items) {
  const size = items.length;
  const indices = Array.from({ length: size }, (_, index) => index);
  const nodes = new Array(size).fill(0);
  const lefts = new Array(size).fill(0);
  const rights = new Array(size).fill(0);
  const mus = new Array(size).fill(0);
  const distances = new Array(size).fill(0);
  const stack = [0, 0, size];
  let created = 0;

  while (stack.length) {
    const hiInitial = stack.pop();
    const lo = stack.pop();
    const nodeIndex = stack.pop();
    let hi = hiInitial;
    const vantagePoint = indices[hi - 1];
    hi--;
    const length = hi - lo;
    nodes[nodeIndex] = vantagePoint;
    if (length === 0) continue;
    if (length === 1) {
      mus[nodeIndex] = distance(items[vantagePoint], items[indices[lo]]);
      created++;
      rights[nodeIndex] = created;
      nodes[created] = indices[lo];
      continue;
    }
    for (let i = lo; i < hi; i++) distances[indices[i]] = distance(items[vantagePoint], items[indices[i]]);
    quickSortIndices(distances, indices, lo, hi);
    const medianIndex = lo + length / 2 - 1;
    const mu = Number.isInteger(medianIndex) ?
      (distances[indices[medianIndex]] + distances[indices[medianIndex + 1]]) / 2 :
      distances[indices[Math.ceil(medianIndex)]];
    mus[nodeIndex] = mu;
    const mid = lowerBound(distances, indices, mu, lo, hi);
    if (hi - mid > 0) {
      created++;
      rights[nodeIndex] = created;
      stack.push(created, mid, hi);
    }
    if (mid - lo > 0) {
      created++;
      lefts[nodeIndex] = created;
      stack.push(created, lo, mid);
    }
  }

  const Pointer = pointerType(size);
  return { nodes: new Pointer(nodes), lefts: new Pointer(lefts), rights: new Pointer(rights), mus: new Float64Array(mus) };
}

function compare(a, b) {
  return a.distance < b.distance ? 1 : a.distance > b.distance ? -1 : 0;
}

function siftDown(heap, item) {
  let index = heap.length;
  heap.push(item);
  while (index > 0) {
    const parent = (index - 1) >> 1;
    if (compare(item, heap[parent]) < 0) {
      heap[index] = heap[parent];
      index = parent;
    }
    else break;
  }
  heap[index] = item;
}

function siftUp(heap, index) {
  const end = heap.length;
  const item = heap[index];
  const start = index;
  let child = index * 2 + 1;
  while (child < end) {
    const right = child + 1;
    if (right < end && compare(heap[child], heap[right]) >= 0) child = right;
    heap[index] = heap[child];
    index = child;
    child = index * 2 + 1;
  }
  heap[index] = item;
  while (index > start) {
    const parent = (index - 1) >> 1;
    if (compare(item, heap[parent]) < 0) {
      heap[index] = heap[parent];
      index = parent;
    }
    else break;
  }
  heap[index] = item;
}

function pop(heap) {
  const last = heap.pop();
  if (heap.length) {
    const item = heap[0];
    heap[0] = last;
    siftUp(heap, 0);
    return item;
  }
  return last;
}

function VPTree(distance, items) {
  if (typeof distance !== 'function') throw new Error('mnemonist/VPTree.constructor: given `distance` must be a function.');
  if (!items) throw new Error('mnemonist/VPTree.constructor: you must provide items to the tree. A VPTree cannot be updated after its creation.');
  this.distance = distance;
  this.items = Array.from(items);
  this._inner = protocol && distance === standardLevenshtein && this.items.every(item => typeof item === 'string')
    ? protocol.create('vp-tree', [this.items])
    : null;
  this.size = this.items.length;
  this.D = 0;
  const tree = createBinaryTree(distance, this.items);
  this.nodes = tree.nodes;
  this.lefts = tree.lefts;
  this.rights = tree.rights;
  this.mus = tree.mus;
}

VPTree.prototype.nearestNeighbors = function(k, query) {
  if (this._inner) return this._inner.call('nearestNeighbors', [k, query]);
  const heap = [];
  const stack = [0];
  let tau = Infinity;
  this.D = 0;
  while (stack.length) {
    const node = stack.pop();
    const item = this.items[this.nodes[node]];
    const d = this.distance(item, query);
    this.D++;
    if (d < tau) {
      siftDown(heap, { distance: d, item });
      if (heap.length > k) pop(heap);
      if (heap.length >= k) tau = heap[0].distance;
    }
    const left = this.lefts[node];
    const right = this.rights[node];
    if (!left && !right) continue;
    const mu = this.mus[node];
    if (d < mu) {
      if (left && d < mu + tau) stack.push(left);
      if (right && d >= mu - tau) stack.push(right);
    }
    else {
      if (right && d >= mu - tau) stack.push(right);
      if (left && d < mu + tau) stack.push(left);
    }
  }
  const result = new Array(heap.length);
  for (let i = heap.length - 1; i >= 0; i--) result[i] = pop(heap);
  return result;
};

VPTree.prototype.neighbors = function(radius, query) {
  if (this._inner) return this._inner.call('neighbors', [radius, query]);
  const neighbors = [];
  const stack = [0];
  this.D = 0;
  while (stack.length) {
    const node = stack.pop();
    const item = this.items[this.nodes[node]];
    const d = this.distance(item, query);
    this.D++;
    if (d <= radius) neighbors.push({ distance: d, item });
    const left = this.lefts[node];
    const right = this.rights[node];
    if (!left && !right) continue;
    const mu = this.mus[node];
    if (d < mu) {
      if (left && d < mu + radius) stack.push(left);
      if (right && d >= mu - radius) stack.push(right);
    }
    else {
      if (right && d >= mu - radius) stack.push(right);
      if (left && d < mu + radius) stack.push(left);
    }
  }
  return neighbors;
};

VPTree.from = function(iterable, distance) { return new VPTree(distance, iterable); };

module.exports = VPTree;
