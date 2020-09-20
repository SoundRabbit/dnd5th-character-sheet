use kagura::prelude::*;

pub struct Props {}

pub enum Sub {}

pub type Exprerience = Component<Props, Sub>;

pub fn new() -> Exprerience {
    Component::new(init, update, render)
}

struct State {}

enum Msg {}

fn init(_: Option<State>, _: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {};
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(_: &State, children: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("key-value"),
        Events::new(),
        children,
    )
}
