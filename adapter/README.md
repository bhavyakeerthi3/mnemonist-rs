# Thin JS Adapter

The root `tests/*.js` files are shims that keep Mnemonist's CommonJS constructor style while delegating to Rust through the optional N-API binding in `src/native.rs`.

Default N-API bridge coverage:

- Stack
- Queue
- LinkedList
- FixedStack
- FixedDeque
- CircularBuffer
- Heap / MaxHeap default comparator path
- Set helper functions

Commands:

```bash
npm run build:native
npm run test:original
```

The bridge type-checks with `cargo check -Fnodejs`. `npm run build:native` builds
the addon on the supported local toolchain.

## Standalone Protocol Mode

`src/main.rs` builds a Node-free, persistent JSONL runner for Stack, Queue,
LinkedList, FixedStack, FixedDeque, CircularBuffer, LRUCache/LRUMap and delete-capable variants, DefaultMap/FuzzyMap/FuzzyMultiMap state, SparseSet, SparseQueueSet, BitSet, BitVector, Vector, SparseMap, numeric KDTree queries, string-native `leven` VPTree/BKTree/PassjoinIndex, StaticDisjointSet, StaticIntervalTree, InvertedIndex, SymSpell, MultiArray, MultiSet, MultiMap, BiMap, CritBitTreeMap, FixedCritBitTreeMap, Heap, FixedReverseHeap, FibonacciHeap, HashedArrayTree, insertion/quick sort helpers, Set helpers, BloomFilter, string-native Trie/TrieMap, and string-native SuffixArray/GeneralizedSuffixArray. `MNEMONIST_TRANSPORT=protocol` switches those root shims to a
synchronous replay transport that invokes only `target/release/mnemonist`; it
does not load `mnemonist.node`. This lets all 499 active preserved upstream assertions run
against the executable:

```bash
npm run test:original:standalone
```

The replay transport is intentionally limited to JSON-safe values and is not a
benchmark path. Opaque handles let Rust own object-identity collection state,
including DefaultWeakMap's observable map operations, but do not emulate host
GC liveness. Callback and exact typed-array APIs remain explicit compatibility
paths until they have native Rust representations.
