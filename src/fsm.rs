use crate::error::FsmError;
use crate::event::Event;

#[derive(Default)]
pub struct State {
    pub name: String,
    pub enter: Option<fn()>,
    pub leave: Option<fn()>,
}

/*
usage:

let mut fsm = FSM::new();
fsm.event("v1").from(state::1).to(state::2).action(|| {

});

fsm.run();


older:

let state = v1;
if state == v1 {
    do_v1();
} else if state == v2 {
    do_v2();
} else if state == v3 {
    do_v3();
}

*/

pub struct FSM {
    name: String,
    initial_state: Option<fn()>,
    final_state: Option<fn()>,
    events: Vec<Event>,
    states: Vec<State>,
}

impl FSM {
    fn current() -> String {
        unimplemented!()
    }

    fn is(state: String) -> bool {
        unimplemented!()
    }

    fn set_state(state: String) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn same_state() {}
}
