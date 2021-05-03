use rust_fsm::*;

state_machine! {
    derive(Debug)
    pub BasicAi(Idle)
    Idle(NoticeHostile) => Attack,
    Attack(CantSeeHostile) => Idle
}

pub fn handle(state: &BasicAiState, machine: &mut StateMachine<BasicAi>) {
    match state {
        BasicAiState::Idle => {
            println!("idle");
            machine.consume(&BasicAiInput::NoticeHostile).unwrap();
        }
        BasicAiState::Attack => (println!("Attack!! :O")),
    }
}
