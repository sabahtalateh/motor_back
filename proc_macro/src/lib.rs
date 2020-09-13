use slog::Logger;

pub trait Service2 {
    fn name() -> String;
}

pub trait HasLogger {
    fn logger(&self) -> &Logger;
}
