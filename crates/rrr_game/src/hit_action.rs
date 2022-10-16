use rrr_input::KeyCode;

#[derive(Debug)]
pub struct Action {
    pub key: KeyCode,
    pub ts: u32,
}

pub struct NeedsKeyCode();
pub struct NeedsTimestamp(KeyCode);

pub trait BuilderMode {}
impl BuilderMode for NeedsKeyCode {}
impl BuilderMode for NeedsTimestamp {}

#[derive(Debug, Default)]
pub struct Builder<S: BuilderMode> {
    inner: S,
}

impl Builder<NeedsKeyCode> {
    #[must_use]
    pub fn with_key_code(key_code: KeyCode) -> Builder<NeedsTimestamp> {
        Builder {
            inner: NeedsTimestamp(key_code),
        }
    }
}

impl Builder<NeedsTimestamp> {
    #[must_use]
    pub fn build(self, ts: u32) -> Action {
        Action {
            key: self.inner.0,
            ts,
        }
    }
}
