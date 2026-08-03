exports.DEFAULT_COMPARATOR = function(a, b) {
  if (a < b) return -1;
  if (a > b) return 1;
  return 0;
};

exports.createTupleComparator = function(size) {
  return function(a, b) {
    for (let i = 0; i < size; i++) {
      if (a[i] < b[i]) return -1;
      if (a[i] > b[i]) return 1;
    }
    return 0;
  };
};
