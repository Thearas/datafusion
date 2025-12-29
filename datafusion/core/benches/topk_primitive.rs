// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing,
//   software distributed under the License is distributed on an
//   "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
//   KIND, either express or implied.  See the License for the
//   specific language governing permissions and limitations
// under the License.

//! Benchmarks for TopK sort/limit optimization with primitive types

use arrow::array::{ArrayRef, Float64Array, Int32Array, Int64Array, RecordBatch, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow_schema::SortOptions;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use datafusion_execution::runtime_env::RuntimeEnv;
use datafusion_physical_expr::expressions::col;
use datafusion_physical_expr_common::sort_expr::{LexOrdering, PhysicalSortExpr};
use datafusion_physical_plan::metrics::ExecutionPlanMetricsSet;
use datafusion_physical_plan::{TopK, TopKDynamicFilters};
use datafusion_physical_expr::expressions::{DynamicFilterPhysicalExpr, lit};
use parking_lot::RwLock;
use std::hint::black_box;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Create test data with the specified number of rows
fn create_int32_batch(num_rows: usize) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new("col", DataType::Int32, false)]));
    
    // Create somewhat random data by using index-based values
    let values: Vec<i32> = (0..num_rows)
        .map(|i| ((i * 7919) % 1000000) as i32) // pseudo-random distribution
        .collect();
    
    let array: ArrayRef = Arc::new(Int32Array::from(values));
    RecordBatch::try_new(schema, vec![array]).unwrap()
}

fn create_int64_batch(num_rows: usize) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new("col", DataType::Int64, false)]));
    let values: Vec<i64> = (0..num_rows)
        .map(|i| ((i * 7919) % 1000000) as i64)
        .collect();
    let array: ArrayRef = Arc::new(Int64Array::from(values));
    RecordBatch::try_new(schema, vec![array]).unwrap()
}

fn create_float64_batch(num_rows: usize) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new("col", DataType::Float64, false)]));
    let values: Vec<f64> = (0..num_rows)
        .map(|i| ((i * 7919) % 1000000) as f64 / 1000.0)
        .collect();
    let array: ArrayRef = Arc::new(Float64Array::from(values));
    RecordBatch::try_new(schema, vec![array]).unwrap()
}

fn create_uint64_batch(num_rows: usize) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![Field::new("col", DataType::UInt64, false)]));
    let values: Vec<u64> = (0..num_rows)
        .map(|i| ((i * 7919) % 1000000) as u64)
        .collect();
    let array: ArrayRef = Arc::new(UInt64Array::from(values));
    RecordBatch::try_new(schema, vec![array]).unwrap()
}

fn create_multi_column_batch(num_rows: usize) -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![
        Field::new("col1", DataType::Int32, false),
        Field::new("col2", DataType::Int32, false),
    ]));
    let values1: Vec<i32> = (0..num_rows)
        .map(|i| ((i * 7919) % 1000000) as i32)
        .collect();
    let values2: Vec<i32> = (0..num_rows)
        .map(|i| ((i * 3541) % 1000000) as i32)
        .collect();
    let array1: ArrayRef = Arc::new(Int32Array::from(values1));
    let array2: ArrayRef = Arc::new(Int32Array::from(values2));
    RecordBatch::try_new(schema, vec![array1, array2]).unwrap()
}

/// Benchmark TopK with Int32
fn bench_topk_int32(c: &mut Criterion, k: usize, num_rows: usize) {
    let rt = Runtime::new().unwrap();
    let batch = create_int32_batch(num_rows);
    let schema = batch.schema();
    
    c.bench_with_input(
        BenchmarkId::new("topk_int32", format!("k={}_rows={}", k, num_rows)),
        &batch,
        |b, batch| {
            b.iter(|| {
                rt.block_on(async {
                    let sort_expr = PhysicalSortExpr {
                        expr: col("col", schema.as_ref()).unwrap(),
                        options: SortOptions::default(),
                    };
                    let expr = LexOrdering::from([sort_expr]);
                    let runtime = Arc::new(RuntimeEnv::default());
                    let metrics = ExecutionPlanMetricsSet::new();
                    let filter = Arc::new(RwLock::new(TopKDynamicFilters::new(Arc::new(
                        DynamicFilterPhysicalExpr::new(vec![], lit(true)),
                    ))));
                    
                    let mut topk = TopK::try_new(
                        0,
                        Arc::clone(&schema),
                        vec![],
                        expr,
                        k,
                        1024,
                        runtime,
                        &metrics,
                        filter,
                    ).unwrap();
                    
                    topk.insert_batch(batch.clone()).unwrap();
                    black_box(topk.emit().unwrap());
                })
            })
        },
    );
}

/// Benchmark TopK with Int64
fn bench_topk_int64(c: &mut Criterion, k: usize, num_rows: usize) {
    let rt = Runtime::new().unwrap();
    let batch = create_int64_batch(num_rows);
    let schema = batch.schema();
    
    c.bench_with_input(
        BenchmarkId::new("topk_int64", format!("k={}_rows={}", k, num_rows)),
        &batch,
        |b, batch| {
            b.iter(|| {
                rt.block_on(async {
                    let sort_expr = PhysicalSortExpr {
                        expr: col("col", schema.as_ref()).unwrap(),
                        options: SortOptions::default(),
                    };
                    let expr = LexOrdering::from([sort_expr]);
                    let runtime = Arc::new(RuntimeEnv::default());
                    let metrics = ExecutionPlanMetricsSet::new();
                    let filter = Arc::new(RwLock::new(TopKDynamicFilters::new(Arc::new(
                        DynamicFilterPhysicalExpr::new(vec![], lit(true)),
                    ))));
                    
                    let mut topk = TopK::try_new(
                        0,
                        Arc::clone(&schema),
                        vec![],
                        expr,
                        k,
                        1024,
                        runtime,
                        &metrics,
                        filter,
                    ).unwrap();
                    
                    topk.insert_batch(batch.clone()).unwrap();
                    black_box(topk.emit().unwrap());
                })
            })
        },
    );
}

/// Benchmark TopK with Float64
fn bench_topk_float64(c: &mut Criterion, k: usize, num_rows: usize) {
    let rt = Runtime::new().unwrap();
    let batch = create_float64_batch(num_rows);
    let schema = batch.schema();
    
    c.bench_with_input(
        BenchmarkId::new("topk_float64", format!("k={}_rows={}", k, num_rows)),
        &batch,
        |b, batch| {
            b.iter(|| {
                rt.block_on(async {
                    let sort_expr = PhysicalSortExpr {
                        expr: col("col", schema.as_ref()).unwrap(),
                        options: SortOptions::default(),
                    };
                    let expr = LexOrdering::from([sort_expr]);
                    let runtime = Arc::new(RuntimeEnv::default());
                    let metrics = ExecutionPlanMetricsSet::new();
                    let filter = Arc::new(RwLock::new(TopKDynamicFilters::new(Arc::new(
                        DynamicFilterPhysicalExpr::new(vec![], lit(true)),
                    ))));
                    
                    let mut topk = TopK::try_new(
                        0,
                        Arc::clone(&schema),
                        vec![],
                        expr,
                        k,
                        1024,
                        runtime,
                        &metrics,
                        filter,
                    ).unwrap();
                    
                    topk.insert_batch(batch.clone()).unwrap();
                    black_box(topk.emit().unwrap());
                })
            })
        },
    );
}

/// Benchmark TopK with UInt64
fn bench_topk_uint64(c: &mut Criterion, k: usize, num_rows: usize) {
    let rt = Runtime::new().unwrap();
    let batch = create_uint64_batch(num_rows);
    let schema = batch.schema();
    
    c.bench_with_input(
        BenchmarkId::new("topk_uint64", format!("k={}_rows={}", k, num_rows)),
        &batch,
        |b, batch| {
            b.iter(|| {
                rt.block_on(async {
                    let sort_expr = PhysicalSortExpr {
                        expr: col("col", schema.as_ref()).unwrap(),
                        options: SortOptions::default(),
                    };
                    let expr = LexOrdering::from([sort_expr]);
                    let runtime = Arc::new(RuntimeEnv::default());
                    let metrics = ExecutionPlanMetricsSet::new();
                    let filter = Arc::new(RwLock::new(TopKDynamicFilters::new(Arc::new(
                        DynamicFilterPhysicalExpr::new(vec![], lit(true)),
                    ))));
                    
                    let mut topk = TopK::try_new(
                        0,
                        Arc::clone(&schema),
                        vec![],
                        expr,
                        k,
                        1024,
                        runtime,
                        &metrics,
                        filter,
                    ).unwrap();
                    
                    topk.insert_batch(batch.clone()).unwrap();
                    black_box(topk.emit().unwrap());
                })
            })
        },
    );
}

/// Benchmark TopK with multi-column (should use generic path)
fn bench_topk_multi_column(c: &mut Criterion, k: usize, num_rows: usize) {
    let rt = Runtime::new().unwrap();
    let batch = create_multi_column_batch(num_rows);
    let schema = batch.schema();
    
    c.bench_with_input(
        BenchmarkId::new("topk_multi_column", format!("k={}_rows={}", k, num_rows)),
        &batch,
        |b, batch| {
            b.iter(|| {
                rt.block_on(async {
                    let sort_expr1 = PhysicalSortExpr {
                        expr: col("col1", schema.as_ref()).unwrap(),
                        options: SortOptions::default(),
                    };
                    let sort_expr2 = PhysicalSortExpr {
                        expr: col("col2", schema.as_ref()).unwrap(),
                        options: SortOptions::default(),
                    };
                    let expr = LexOrdering::from([sort_expr1, sort_expr2]);
                    let runtime = Arc::new(RuntimeEnv::default());
                    let metrics = ExecutionPlanMetricsSet::new();
                    let filter = Arc::new(RwLock::new(TopKDynamicFilters::new(Arc::new(
                        DynamicFilterPhysicalExpr::new(vec![], lit(true)),
                    ))));
                    
                    let mut topk = TopK::try_new(
                        0,
                        Arc::clone(&schema),
                        vec![],
                        expr,
                        k,
                        1024,
                        runtime,
                        &metrics,
                        filter,
                    ).unwrap();
                    
                    topk.insert_batch(batch.clone()).unwrap();
                    black_box(topk.emit().unwrap());
                })
            })
        },
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    let k_values = vec![10, 100];
    let row_counts = vec![1000, 10000];
    
    for &k in &k_values {
        for &rows in &row_counts {
            bench_topk_int32(c, k, rows);
            bench_topk_int64(c, k, rows);
            bench_topk_float64(c, k, rows);
            bench_topk_uint64(c, k, rows);
            bench_topk_multi_column(c, k, rows);
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
