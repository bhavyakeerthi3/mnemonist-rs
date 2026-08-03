'use strict';

exports.indices = function(length) {
  const ArrayClass = length <= 0xff ? Uint8Array : length <= 0xffff ? Uint16Array : Uint32Array;
  const indices = new ArrayClass(length);
  for (let i = 0; i < length; i++) indices[i] = i;
  return indices;
};
