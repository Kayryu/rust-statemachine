#[derive(Default)]
pub struct Event {
    name: String,
    src: String,
    dst: String,
    before: Option<fn()>,
    after: Option<fn()>,
    is_canceled: bool,
    is_async: bool,
}
