use super::key_value;
use crate::model::character;
use crate::util::prop::R;
use kagura::prelude::*;

pub struct Props {
    pub growth: R<character::Growth>,
}

pub enum Sub {}

pub type Growth = Component<Props, Sub>;

pub fn new() -> Growth {
    Component::new(init, update, render)
}

struct State {
    growth: R<character::Growth>,
    is_collapsed: bool,
}

enum Msg {}

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

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
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
        vec![Html::div(
            Attributes::new()
                .class("growth__heading")
                .class("growth__heading--acquisition"),
            Events::new(),
            vec![
                Html::select(
                    Attributes::new(),
                    Events::new(),
                    vec![
                        Html::option(
                            Attributes::new().selected(),
                            Events::new(),
                            vec![Html::text("獲得")],
                        ),
                        Html::option(Attributes::new(), Events::new(), vec![Html::text("消費")]),
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
        vec![Html::div(
            Attributes::new()
                .class("growth__heading")
                .class("growth__heading--consumption"),
            Events::new(),
            vec![
                Html::select(
                    Attributes::new(),
                    Events::new(),
                    vec![
                        Html::option(Attributes::new(), Events::new(), vec![Html::text("獲得")]),
                        Html::option(
                            Attributes::new().selected(),
                            Events::new(),
                            vec![Html::text("消費")],
                        ),
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
