use super::key_value;
use crate::model::character;
use crate::util::prop::{C, R};
use kagura::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

pub struct Props {
    pub growth: R<character::Growth>,
}

pub enum Sub {
    SetGrowth(character::Growth),
}

pub type Growth = Component<Props, Sub>;

pub fn new() -> Growth {
    Component::new(init, update, render)
}

struct State {
    growth: R<character::Growth>,
    is_collapsed: bool,
}

enum Msg {
    SetGrowthType(bool),
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = if let Some(state) = state {
        state
    } else {
        State {
            growth: props.growth,
            is_collapsed: false,
        }
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::SetGrowthType(is_acquisition) => {
            let growth = if is_acquisition {
                character::Growth::Acquisition {
                    title: state.growth.borrow().title().clone(),
                    experience: state.growth.borrow().experience(),
                    description: state.growth.borrow().description().clone(),
                }
            } else {
                character::Growth::Consumption {
                    title: state.growth.borrow().title().clone(),
                    experience: state.growth.borrow().experience(),
                    class_name: String::new(),
                    status: character::Status::new(),
                    description: state.growth.borrow().description().clone(),
                }
            };
            Cmd::sub(Sub::SetGrowth(growth))
        }
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    match &(*state.growth.borrow()) {
        character::Growth::Acquisition {
            title,
            experience,
            description,
        } => acquisition(title, *experience, description),
        character::Growth::Consumption {
            title,
            experience,
            class_name,
            status,
            description,
        } => consumption(title, *experience, class_name, status, description),
    }
}

fn acquisition(title: &String, experience: u32, description: &String) -> Html {
    Html::div(
        Attributes::new().class("growth").class("growth"),
        Events::new(),
        vec![heading(title, experience, true)],
    )
}

fn consumption(
    title: &String,
    experience: u32,
    class_name: &String,
    status: &character::Status,
    description: &String,
) -> Html {
    Html::div(
        Attributes::new().class("growth").class("growth"),
        Events::new(),
        vec![heading(title, experience, false)],
    )
}

fn heading(title: &String, experience: u32, is_acquisition: bool) -> Html {
    let (attr_1, attr_2) = if is_acquisition {
        (
            Attributes::new().value("acquisition").selected(),
            Attributes::new().value("consumption"),
        )
    } else {
        (
            Attributes::new().value("acquisition"),
            Attributes::new().value("consumption").selected(),
        )
    };
    Html::div(
        Attributes::new().class("growth").class("growth"),
        Events::new(),
        vec![Html::div(
            Attributes::new()
                .class("growth__heading")
                .class(if is_acquisition {
                    "growth__heading--acquisition"
                } else {
                    "growth__heading--consumption"
                }),
            Events::new(),
            vec![
                Html::select(
                    Attributes::new(),
                    Events::new().on("change", move |_| Msg::SetGrowthType(!is_acquisition)),
                    vec![
                        Html::option(attr_1, Events::new(), vec![Html::text("獲得")]),
                        Html::option(attr_2, Events::new(), vec![Html::text("消費")]),
                    ],
                ),
                Html::input(
                    Attributes::new()
                        .type_("number")
                        .value(experience.to_string()),
                    Events::new(),
                    vec![],
                ),
                Html::input(
                    Attributes::new()
                        .type_("text")
                        .value(title)
                        .class("growth__title"),
                    Events::new(),
                    vec![],
                ),
            ],
        )],
    )
}
