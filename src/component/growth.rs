use super::function::text;
use super::key_value;
use super::select;
use crate::model::character;
use crate::util::prop::R;
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
    NoOp,
    SetGrowthType(bool),
    SetExperience(u32),
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
        Msg::NoOp => Cmd::none(),
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
        Msg::SetExperience(exp) => {
            let mut growth = state.growth.borrow().clone();
            match &mut growth {
                character::Growth::Acquisition { experience, .. }
                | character::Growth::Consumption { experience, .. } => {
                    *experience = exp;
                }
            }
            Cmd::Sub(Sub::SetGrowth(growth))
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
        vec![
            heading(title, experience, true),
            Html::div(
                Attributes::new()
                    .class("growth__data")
                    .class("growth__data--acquisition"),
                Events::new(),
                vec![
                    Html::h4(Attributes::new(), Events::new(), vec![Html::text("メモ")]),
                    Html::textarea(Attributes::new().value(description), Events::new(), vec![]),
                ],
            ),
        ],
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
        vec![
            heading(title, experience, false),
            Html::div(
                Attributes::new()
                    .class("growth__data")
                    .class("growth__data--consumption"),
                Events::new(),
                vec![
                    Html::h4(Attributes::new(), Events::new(), vec![Html::text("成長")]),
                    Html::div(
                        Attributes::new().class("growth__list"),
                        Events::new(),
                        vec![
                            vec![
                                Html::div(
                                    Attributes::new().class("growth__list-item-5"),
                                    Events::new(),
                                    vec![Html::text("クラス")],
                                ),
                                Html::div(
                                    Attributes::new().class("growth__list-item-5"),
                                    Events::new(),
                                    vec![Html::component(
                                        select::new().with(select::Props {
                                            suid: crate::suid!(),
                                            option: vec![
                                                String::from("ウィザード"),
                                                String::from("ウォーロック"),
                                                String::from("クレリック"),
                                                String::from("ソーサラー"),
                                                String::from("ドルイド"),
                                                String::from("バード"),
                                                String::from("バーバリアン"),
                                                String::from("パラディン"),
                                                String::from("ファイター"),
                                                String::from("モンク"),
                                                String::from("レンジャー"),
                                                String::from("ローグ"),
                                            ],
                                        }),
                                        vec![],
                                    )],
                                ),
                            ],
                            input_growth_of_status("HP上昇", 0),
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                    ),
                    Html::div(
                        Attributes::new().class("growth__list"),
                        Events::new(),
                        vec![
                            input_growth_of_status("筋力", status.strength),
                            input_growth_of_status("敏捷力", status.dexterity),
                            input_growth_of_status("耐久力", status.constitution),
                            input_growth_of_status("知力", status.intelligence),
                            input_growth_of_status("判断力", status.wisdom),
                            input_growth_of_status("魅力", status.charisma),
                        ]
                        .into_iter()
                        .flatten()
                        .collect(),
                    ),
                    Html::h4(Attributes::new(), Events::new(), vec![Html::text("メモ")]),
                    Html::textarea(Attributes::new().value(description), Events::new(), vec![]),
                ],
            ),
        ],
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
                    Events::new().on_input(|exp| {
                        if let Ok(exp) = exp.parse() {
                            Msg::SetExperience(exp)
                        } else {
                            Msg::NoOp
                        }
                    }),
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
                Html::button(
                    Attributes::new()
                        .class("pure-button")
                        .class("growth__remove-button"),
                    Events::new(),
                    vec![Html::text("×")],
                ),
            ],
        )],
    )
}

fn input_growth_of_status(status_name: impl Into<String>, value: u32) -> Vec<Html> {
    vec![
        text::div(status_name),
        Html::input(
            Attributes::new().value(value.to_string()).type_("number"),
            Events::new(),
            vec![],
        ),
    ]
}
