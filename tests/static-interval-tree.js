'use strict';

const forEach = require('obliterator/foreach');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function StaticIntervalTree(intervals, getters) {
  this.intervals = intervals;
  this.size = intervals.length;
  this._inner = protocol && !getters ? protocol.create('static-interval-tree', [intervals]) : null;
  if (this._inner) {
    this.height = this._inner.call('height', []);
    return;
  }
  this._start = getters ? getters[0] : interval => interval[0];
  this._end = getters ? getters[1] : interval => interval[1];
  this.height = Math.ceil(Math.log2(this.size + 1));
  this._tree = new Array(Math.max(0, Math.pow(2, this.height) - 1)).fill(-1);
  this._maxEnds = new Array(this.size);
  const indices = intervals.map((_, index) => index).sort((a, b) => this._start(intervals[a]) - this._start(intervals[b]));

  const build = (node, low, high) => {
    const mid = Math.floor(low + (high - low) / 2);
    const index = indices[mid];
    this._tree[node] = index;
    let maxIndex = index;
    if (low < mid) {
      const left = build(node * 2 + 1, low, mid - 1);
      if (this._end(intervals[left]) > this._end(intervals[maxIndex])) maxIndex = left;
    }
    if (mid < high) {
      const right = build(node * 2 + 2, mid + 1, high);
      if (this._end(intervals[right]) > this._end(intervals[maxIndex])) maxIndex = right;
    }
    this._maxEnds[index] = maxIndex;
    return maxIndex;
  };

  if (this.size) build(0, 0, this.size - 1);
}

StaticIntervalTree.prototype._query = function(start, end) {
  if (this._inner) {
    return start === end ?
      this._inner.call('queryPoint', [start]) :
      this._inner.call('queryInterval', [start, end]);
  }
  const matches = [];
  if (!this.size) return matches;
  const stack = [0];
  while (stack.length) {
    const node = stack.pop();
    const index = this._tree[node];
    if (index < 0) continue;
    if (start > this._end(this.intervals[this._maxEnds[index]])) continue;
    const left = node * 2 + 1;
    if (left < this._tree.length && this._tree[left] >= 0) stack.push(left);
    const interval = this.intervals[index];
    const intervalStart = this._start(interval);
    const intervalEnd = this._end(interval);
    if (end >= intervalStart && start <= intervalEnd) matches.push(interval);
    if (end < intervalStart) continue;
    const right = node * 2 + 2;
    if (right < this._tree.length && this._tree[right] >= 0) stack.push(right);
  }
  return matches;
};

StaticIntervalTree.prototype.intervalsContainingPoint = function(point) {
  return this._query(point, point);
};

StaticIntervalTree.prototype.intervalsOverlappingInterval = function(interval) {
  if (this._inner) return this._query(interval[0], interval[1]);
  return this._query(this._start(interval), this._end(interval));
};

StaticIntervalTree.from = function(iterable, getters) {
  const intervals = [];
  forEach(iterable, (value, key) => {
    intervals.push(Array.isArray(value) || getters ? value : [key, value]);
  });
  return new StaticIntervalTree(intervals, getters);
};

module.exports = StaticIntervalTree;
