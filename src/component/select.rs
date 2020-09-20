use crate::util::luid;
use kagura::prelude::*;

pub struct Props {
    pub suid: u64,
    pub option: Vec<String>,
}

pub enum Sub {}

pub type Select = Component<Props, Sub>;

pub fn new() -> Select {
    Component::new(init, update, render)
}

struct State {
    id: String,
    option: Vec<String>,
    manual: Option<String>,
}

enum Msg {}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let mut state = if let Some(state) = state {
        state
    } else {
        State {
            id: String::from("select__") + &props.suid.to_string(),
            option: vec![],
            manual: None,
        }
    };
    state.option = props.option;

    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State, _: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("select"),
        Events::new(),
        vec![
            Html::input(
                Attributes::new()
                    .string("list", &state.id)
                    .value(state.manual.as_ref().unwrap_or(&String::new()))
                    .class("select__input"),
                Events::new(),
                vec![],
            ),
            Html::datalist(
                Attributes::new().id(&state.id),
                Events::new(),
                state
                    .option
                    .iter()
                    .map(|an_option| {
                        Html::option(Attributes::new().value(an_option), Events::new(), vec![])
                    })
                    .collect(),
            ),
        ],
    )
}
