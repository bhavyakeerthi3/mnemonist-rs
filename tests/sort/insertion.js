'use strict';

const protocol = process.env.MNEMONIST_TRANSPORT === 'protocol'
  ? require('../../adapter/_protocol.js')
  : null;

function inplaceInsertionSort(array, start, end) {
  if (protocol) {
    const sorted = protocol.invoke('sort', 'insertion', [array, start, end]);
    for (let i = 0; i < sorted.length; i++) array[i] = sorted[i];
    return array;
  }
  for (let i = start + 1; i < end; i++) {
    const value = array[i];
    let j = i - 1;
    while (j >= start && array[j] > value) {
      array[j + 1] = array[j];
      j--;
    }
    array[j + 1] = value;
  }
  return array;
}

function inplaceInsertionSortIndices(array, indices, start, end) {
  if (protocol) {
    const sorted = protocol.invoke('sort', 'insertionIndices', [array, Array.from(indices), start, end]);
    for (let i = 0; i < sorted.length; i++) indices[i] = sorted[i];
    return indices;
  }
  for (let i = start + 1; i < end; i++) {
    const index = indices[i];
    let j = i - 1;
    while (j >= start && array[indices[j]] > array[index]) {
      indices[j + 1] = indices[j];
      j--;
    }
    indices[j + 1] = index;
  }
  return indices;
}

exports.inplaceInsertionSort = inplaceInsertionSort;
exports.inplaceInsertionSortIndices = inplaceInsertionSortIndices;
