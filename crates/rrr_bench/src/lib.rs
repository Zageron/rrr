#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct BenchmarkResults {
    pub min_frame_time: f32,
    pub max_frame_time: f32,
    pub avg_frame_time: f32,
}

impl BenchmarkResults {
    pub fn new() -> Self {
        BenchmarkResults {
            min_frame_time: f32::MAX,
            max_frame_time: f32::MIN,
            avg_frame_time: 0.0,
        }
    }
}

#[derive(Default, Debug)]
pub struct BenchmarkData {
    pub frame_times: Vec<f32>,
}

impl BenchmarkData {
    pub fn add_frame_time(&mut self, frame_time: f32) {
        self.frame_times.push(frame_time);
    }
}
