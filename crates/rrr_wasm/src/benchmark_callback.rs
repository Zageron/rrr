use rrr_bench::{BenchmarkData, BenchmarkResults};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "\\/bench.js")]
extern "C" {
    fn update_benchmark_ui(data: BenchmarkResults);
}

pub struct BenchmarkCallback {}

impl BenchmarkCallback {
    pub unsafe fn run(&self, data: &BenchmarkData) {
        unsafe {
            update_benchmark_ui(BenchmarkResults {
                min_frame_time: 0.,
                max_frame_time: 0.,
                avg_frame_time: 0.,
            });
        }
    }
}
