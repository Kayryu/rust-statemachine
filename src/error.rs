pub enum FsmError {
    UnknownEventError,
    InvalidEventError,
    InTransitionError,
    NotInTransitionError,
    NoTransitionError,
    CanceledError,
    AsyncError,
    InternalError,
}
