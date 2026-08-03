const ringProto = require('../adapter/_ring.js');
const { native } = require('../adapter/_helpers.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function FixedDeque(ArrayClass, capacity) {
  if (arguments.length < 2) {
    throw new Error('mnemonist/fixed-deque: expecting an Array class and a capacity.');
  }
  if (typeof capacity !== 'number' || capacity <= 0) {
    throw new Error('mnemonist/fixed-deque: `capacity` should be a positive number.');
  }
  this._inner = protocol
    ? protocol.create('fixed-deque', [capacity])
    : new native.FixedDequeInner(capacity);
  this.capacity = capacity;
  this.ArrayClass = ArrayClass;
}

ringProto(FixedDeque, 'FixedDequeInner', 'fixed-deque');
module.exports = FixedDeque;
