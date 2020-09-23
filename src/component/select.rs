use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub suid: u64,
    pub option: Vec<String>,
    pub selected: String,
}

pub enum Sub {
    ChangeValue(String),
}

pub type Select = Component<Props, Sub>;

pub fn new() -> Select {
    Component::new(init, update, render)
}

struct State {
    id: String,
    option: Vec<String>,
    selected: String,
}

enum Msg {
    NoOp,
    ChangeValue(String),
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = if let Some(mut state) = state {
        state.option = props.option;
        state.selected = props.selected;
        state
    } else {
        State {
            id: String::from("select__") + &props.suid.to_string(),
            option: props.option,
            selected: props.selected,
        }
    };

    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::ChangeValue(selected) => {
            state.selected = selected.clone();
            Cmd::sub(Sub::ChangeValue(selected))
        }
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("select"),
        Events::new(),
        vec![
            Html::input(
                Attributes::new()
                    .string("list", &state.id)
                    .value(&state.selected)
                    .class("select__input"),
                Events::new().on("change", |e| {
                    e.target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                        .map(|t| Msg::ChangeValue(t.value()))
                        .unwrap_or(Msg::NoOp)
                }),
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
