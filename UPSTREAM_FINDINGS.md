# Upstream Behavioral Findings

These findings came from differential testing against the preserved upstream
source in `original/mnemonist/`. The hashed test suite remains unmodified.

## LinkedList keeps a stale tail after its final shift

In the upstream `linked-list.js`, shifting the last remaining node clears
`head` but does not clear `tail`. Consequently, `size` is zero while `last()`
can still return the removed item.

Reproduction from the repository root:

```js
const LinkedList = require('./original/mnemonist/linked-list.js');
const list = new LinkedList();

list.push('removed');
list.shift();

console.log(list.size);   // 0
console.log(list.last()); // 'removed' (stale; an empty list should yield undefined)
```

The Rust port returns `undefined` for the empty JS adapter surface, which is
consistent with `first()` and the public empty-result contract. The differential
harness records this as an upstream-only invalid state rather than copying it
into the port. See `scripts/differential-fuzz.js` and `fuzz/log.txt` for the
reproducible seeded campaign that found it.

## BitVector `select` loses empty 32-bit words

The upstream `BitVector#select` loop advances its position counter only for a
non-zero backing word. A completely empty preceding word is therefore omitted
from the reported index.

```js
const BitVector = require('./original/mnemonist/bit-vector.js');
const vector = new BitVector(68);

vector.flip(50);

console.log(vector.select(1)); // 18, but the sole set bit is at index 50
```

The Rust port returns 50. Differential comparisons of `select` stay within the
upstream-consistent first-word domain; wider bit-vector traces continue to
compare state, rank, values, and entries. The port intentionally does not copy
this incorrect index calculation.

## BitVector `size` can become stale

Upstream `pop()` shortens the vector but does not decrement `size` when the
removed bit is set. Shrinking `resize()` leaves the count of set bits outside
the new logical length in `size`. The cached count can also drift during normal
operations because `reset()` compares a signed bitwise result to an unsigned
backing word.

```js
const BitVector = require('./original/mnemonist/bit-vector.js');
const vector = new BitVector();

vector.push(1);
vector.pop();

console.log(vector.length); // 0
console.log(vector.size);   // 1 (stale; the vector is empty)
```

```js
const vector = new BitVector(32);
vector.set(31);
vector.reset(0); // Bit 0 was already clear.

console.log(vector.size); // 0, while bit 31 remains set
```

The seeded parity campaign excludes `pop` and shrinking `resize`, uses clear
bits to drive capacity growth, restricts set/reset/flip to upstream-safe
low-word indices, compares state, rank, values, entries, logical length, and
capacity, and independently asserts Rust's `size` invariant after every
mutation. Rust keeps its size invariant correct after every mutation rather
than copying stale metadata.

## BitVector iterators can exceed logical length

When capacity has a fully unused backing word, upstream `values()` and
`entries()` iterate that word as though it were part of the logical vector.

```js
const BitVector = require('./original/mnemonist/bit-vector.js');
const vector = new BitVector();

for (let i = 0; i < 97; i++) vector.push(0);

console.log(vector.length);                     // 97
console.log(Array.from(vector.values()).length); // 129
```

The Rust iterator always yields exactly `length` values. The differential
campaign caps push-driven traces at the last upstream-consistent capacity tier
and still exercises growth through 96 bits.

## BitVector `rank(length)` fails at a word boundary

At an exact multiple of 32, upstream `rank()` reads one backing word past the
end while masking its final word, producing zero rather than the number of set
bits.

```js
const BitVector = require('./original/mnemonist/bit-vector.js');
const vector = new BitVector(32);

vector.set(1);

console.log(vector.rank(32)); // 0, expected 1
```

The Rust implementation returns 1. The differential campaign compares rank
only below logical length, where the upstream operation is well-defined.

## LRUCacheWithDelete traversal corrupts after setpop then removal

After replacing an existing key with `setpop`, upstream delete/remove can leave
the linked traversal pointing to the deleted slot and return a duplicate entry.

```js
const Cache = require('./original/mnemonist/lru-cache-with-delete.js');
const cache = new Cache(8);

cache.set('theta', 42);
cache.set('delta', 'alpha');
cache.set('beta', 'alpha');
cache.setpop('delta', 0);
cache.remove('delta');

console.log(Array.from(cache.entries()));
// [['beta', 'alpha'], ['beta', 'alpha']]
// Expected: [['beta', 'alpha'], ['theta', 42]]
```

The Rust cache returns the expected surviving entries. The strict
LRUCacheWithDelete differential trace covers set, setpop, get, peek, clear,
has, values, keys, and entries; removal after in-place setpop is retained as an
upstream finding rather than copied into the port.
