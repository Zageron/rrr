mod imp;
pub use imp::platform::Time;

pub trait TimeTrait: Copy {
    fn now() -> Self;
    fn ms_since(&self) -> f64;
    fn sub(&self, other: &Self) -> f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    struct TimeContainer<T: TimeTrait> {
        start_instant: T,
        end_instant: T,
    }

    impl<T> TimeContainer<T>
    where
        T: TimeTrait,
    {
        pub fn new() -> Self {
            Self {
                start_instant: T::now(),
                end_instant: T::now(),
            }
        }
    }

    fn impl_test_time() {
        let mut time_container = TimeContainer::<Time>::new();
        for _i in 0..10000 {}
        time_container.end_instant = Time::now();
        time_container.start_instant.ms_since();
        time_container
            .end_instant
            .sub(&time_container.start_instant);
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test(async)]
    async fn test_time() {
        impl_test_time()
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_time() {
        impl_test_time()
    }
}
