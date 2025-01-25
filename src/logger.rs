/// Generic way to build logs on the platform
///
#[derive(Clone)]
pub struct Logger {
    pub class: String,
    pub i1: String,
    pub i2: String,
    pub i3: String,
    pub plugin: String,
}


impl Logger {

    /// Create a new logger
    ///
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>, D: Into<String>>(
        class: A,
        i1: B,
        i2: C,
        i3: D,
    ) -> Logger {
        return Logger {
            class: class.into(),
            i1: i1.into(),
            i2: i2.into(),
            i3: i3.into(),
            plugin: String::new(),
        };
    }

    ///
    ///
    pub fn new_for_crate() -> Self {
        Self::new("Crate", "usbtmc-message", "", "")
    }

    pub fn error<A: Into<String>>(&self, text: A) {
        tracing::error!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn warn<A: Into<String>>(&self, text: A) {
        tracing::warn!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn info<A: Into<String>>(&self, text: A) {
        tracing::info!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn debug<A: Into<String>>(&self, text: A) {
        tracing::debug!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }

    pub fn trace<A: Into<String>>(&self, text: A) {
        tracing::trace!(
            class = self.class,
            i1 = self.i1,
            i2 = self.i2,
            i3 = self.i3,
            plugin = self.plugin,
            "{}",
            text.into()
        );
    }
}
