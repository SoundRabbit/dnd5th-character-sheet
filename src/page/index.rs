use kagura::prelude::*;

pub struct Props {}

pub enum Sub {}

pub fn new() -> Component<Props, Sub> {
    Component::new(init, update, render)
}

struct State {}

enum Msg {}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {};
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State, children: Vec<Html>) -> Html {
    Html::text("It works")
}
