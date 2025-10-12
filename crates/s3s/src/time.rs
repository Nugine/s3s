use std::fmt;

pub fn now_utc() -> impl fmt::Debug {
    #[cfg(not(target_arch = "wasm32"))]
    {
        time::OffsetDateTime::now_utc()
    }
    #[cfg(target_arch = "wasm32")]
    {
        ()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Instant(std::time::Instant);

#[cfg(target_arch = "wasm32")]
pub struct Instant(());

impl Instant {
    pub fn now() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self(std::time::Instant::now())
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self(())
        }
    }

    pub fn elapsed(&self) -> impl fmt::Debug {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.0.elapsed()
        }
        #[cfg(target_arch = "wasm32")]
        {
            ()
        }
    }
}
