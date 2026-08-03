const { native, addIterableMethods } = require('../adapter/_helpers.js');
const ringProto = require('../adapter/_ring.js');
const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../adapter/_protocol.js')
  : null;

function CircularBuffer(ArrayClass, capacity) {
  if (arguments.length < 2) {
    throw new Error('mnemonist/circular-buffer: expecting an Array class and a capacity.');
  }
  if (typeof capacity !== 'number' || capacity <= 0) {
    throw new Error('mnemonist/circular-buffer: `capacity` should be a positive number.');
  }
  this._inner = protocol
    ? protocol.create('circular-buffer', [capacity])
    : new native.CircularBufferInner(capacity);
  this.capacity = capacity;
  this.ArrayClass = ArrayClass;
}

ringProto(CircularBuffer, 'CircularBufferInner', 'circular-buffer');
module.exports = CircularBuffer;
