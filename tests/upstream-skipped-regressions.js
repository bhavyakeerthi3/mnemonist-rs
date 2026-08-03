'use strict';

const assert = require('assert');
const SuffixArray = require('./suffix-array.js');
const GeneralizedSuffixArray = SuffixArray.GeneralizedSuffixArray;

describe('Upstream-skipped regressions', function() {
  it('handles issue #196 string tokens', function() {
    const suffixArray = new GeneralizedSuffixArray([
      '1234',
      '234',
      '1234'
    ]);

    assert.deepStrictEqual(suffixArray.longestCommonSubsequence(), '234');
  });

  it('handles issue #196 integer tokens', function() {
    const suffixArray = new GeneralizedSuffixArray([
      [1, 2, 3, 4],
      [2, 3, 4],
      [1, 2, 3, 4]
    ]);

    assert.deepStrictEqual(suffixArray.longestCommonSubsequence(), [2, 3, 4]);
  });
});
