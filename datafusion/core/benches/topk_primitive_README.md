# TopK Primitive Type Optimization Benchmark

This benchmark measures the performance improvement from the TopK optimization for single-column primitive types.

## Running the Benchmark

To run the benchmark:

```bash
cd datafusion/core
cargo bench --bench topk_primitive
```

## What it Measures

The benchmark tests TopK performance with:
- **Int32**: 32-bit signed integers
- **Int64**: 64-bit signed integers  
- **Float64**: 64-bit floating point
- **UInt64**: 64-bit unsigned integers
- **Multi-column**: Two Int32 columns (uses generic path for comparison)

Each test varies:
- **k values**: 10, 100 (number of top elements to keep)
- **row counts**: 1,000, 10,000 (input batch size)

## Expected Results

The optimized single-column primitive types should show performance improvements over the multi-column baseline because they:
1. Skip RowConverter serialization
2. Use inline storage (0 heap bytes vs 20+ bytes per value)
3. Compare using native CPU instructions vs memcmp

Example output:
```
topk_int32/k=10_rows=1000     time:   [XXX μs XXX μs XXX μs]
topk_int64/k=10_rows=1000     time:   [XXX μs XXX μs XXX μs]
topk_multi_column/k=10_rows=1000  time:   [XXX μs XXX μs XXX μs]
```

The single-column primitive benchmarks should be faster than the multi-column benchmark for equivalent k and row counts.
