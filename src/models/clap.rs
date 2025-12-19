pub enum State {
    Create,
    Update,
    Delete,
    GetAll,
    GetSingle,
    Exit,
}

impl State {
    pub fn new(input: &str) -> Self {
        match input.trim() {
            "1" => State::Create,
            "2" => State::Update,
            "3" => State::Delete,
            "4" => State::GetAll,
            "5" => State::GetSingle,
            "6" => State::Exit,
            _ => State::Exit,
        }
    }
}
