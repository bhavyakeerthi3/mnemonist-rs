'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../../adapter/_protocol.js')
  : null;

function sortSlice(array, start, end) {
  if (protocol) {
    const sorted = protocol.invoke('sort', 'quick', [array, start, end]);
    for (let i = 0; i < sorted.length; i++) array[i] = sorted[i];
    return array;
  }
  const sorted = array.slice(start, end).sort((a, b) => a - b);
  for (let i = 0; i < sorted.length; i++) array[start + i] = sorted[i];
  return array;
}

function sortIndices(array, indices, start, end) {
  if (protocol) {
    const sorted = protocol.invoke('sort', 'quickIndices', [array, Array.from(indices), start, end]);
    for (let i = 0; i < sorted.length; i++) indices[i] = sorted[i];
    return indices;
  }
  const sorted = Array.from(indices.slice(start, end)).sort((a, b) => {
    const difference = array[a] - array[b];
    return difference || b - a;
  });
  for (let i = 0; i < sorted.length; i++) indices[start + i] = sorted[i];
  return indices;
}

exports.inplaceQuickSort = sortSlice;
exports.inplaceQuickSortIndices = sortIndices;
