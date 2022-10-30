#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Default, Debug)]
pub struct Bencher {
    bench_data: BenchmarkData,
    previous_tick: u32,
    accumulator: u32,
    cached_times: FrameTimes,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Default, Debug, Clone, Copy)]
pub struct FrameTimes {
    pub avg_frame_time: f64,
    pub one_percent_frame_time: f64,
    pub tenth_percent_frame_time: f64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Default, Debug)]
pub struct BenchmarkData {
    frame_times: Vec<u32>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl Bencher {
    #[cfg_attr(
        target_arch = "wasm32",
        wasm_bindgen::prelude::wasm_bindgen(constructor)
    )]
    pub fn new(now: u32) -> Self {
        Bencher {
            bench_data: BenchmarkData::default(),
            previous_tick: now,
            accumulator: 0,
            cached_times: FrameTimes::default(),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
    pub fn update(&mut self, now: u32) {
        let frame_time = now - self.previous_tick;
        self.add_frame_time(frame_time);

        self.accumulator += frame_time;
        self.previous_tick = now.into();

        if self.accumulator >= 1000 {
            let times = &mut self.bench_data.frame_times;
            times.sort_unstable_by(|x, y| y.cmp(&x));

            let num_times = times.len() as u32;

            let average = times.iter().sum::<u32>() as f64 / num_times as f64;

            let one_takes = u32::max(1, num_times / 100);
            let one_percent =
                times.iter().take(one_takes as usize).sum::<u32>() as f64 / one_takes as f64;

            let tenth_takes = u32::max(1, num_times / 1000);
            let tenth_percent =
                times.iter().take(tenth_takes as usize).sum::<u32>() as f64 / tenth_takes as f64;

            log::info!("samples: {:?}", times);
            log::info!("1% take: {:?}", one_takes);
            log::info!(".1% take: {:?}", one_takes);

            self.cached_times = FrameTimes {
                avg_frame_time: average,
                one_percent_frame_time: one_percent,
                tenth_percent_frame_time: tenth_percent,
            };

            self.accumulator = 0;
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
    pub fn current_data(&self) -> FrameTimes {
        self.cached_times
    }

    pub fn add_frame_time(&mut self, frame_time: u32) {
        self.bench_data.frame_times.push(frame_time);
    }
}
