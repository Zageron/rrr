use rrr_render::Renderer;

#[derive(Default)]
struct RustRustRevolution {
    renderer: Option<Renderer>,
}

impl RustRustRevolution {
    #[must_use]
    pub fn with_renderer(renderer: Renderer) -> Self {
        Self {
            renderer: Some(renderer),
        }
    }
}
