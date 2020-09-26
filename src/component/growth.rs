use super::function::{icon, text};
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
    RemoveSelf,
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
    SetIsCollapsed(bool),
    SetGrowthType(bool),
    SetExperience(u32),
    SetTitle(String),
    SetDescription(String),
    SetClassName(String),
    SetHp(i32),
    SetStatus(character::StatusItem),
    RemoveSelf,
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = if let Some(mut state) = state {
        state.growth = props.growth;
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
        Msg::SetIsCollapsed(is_collapsed) => {
            state.is_collapsed = is_collapsed;
            Cmd::none()
        }
        Msg::SetGrowthType(is_acquisition) => {
            let growth = if is_acquisition {
                character::Growth::Acquisition {
                    title: state.growth.borrow().title().clone(),
                    experience: state.growth.borrow().experience(),
                    description: state.growth.borrow().description().clone(),
                    key: state.growth.borrow().key(),
                }
            } else {
                character::Growth::Consumption {
                    title: state.growth.borrow().title().clone(),
                    experience: state.growth.borrow().experience(),
                    class_name: String::new(),
                    hp: 0,
                    status: character::Status::new(),
                    description: state.growth.borrow().description().clone(),
                    key: state.growth.borrow().key(),
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
        Msg::SetTitle(changed_title) => {
            let mut growth = state.growth.borrow().clone();
            match &mut growth {
                character::Growth::Acquisition { title, .. }
                | character::Growth::Consumption { title, .. } => {
                    *title = changed_title;
                }
            }
            Cmd::Sub(Sub::SetGrowth(growth))
        }
        Msg::SetDescription(changed_description) => {
            let mut growth = state.growth.borrow().clone();
            match &mut growth {
                character::Growth::Acquisition { description, .. }
                | character::Growth::Consumption { description, .. } => {
                    *description = changed_description;
                }
            }
            Cmd::Sub(Sub::SetGrowth(growth))
        }
        Msg::SetClassName(changed_class_name) => {
            let mut growth = state.growth.borrow().clone();
            let is_changed = match &mut growth {
                character::Growth::Consumption { class_name, .. } => {
                    *class_name = changed_class_name;
                    true
                }
                _ => false,
            };
            if is_changed {
                Cmd::sub(Sub::SetGrowth(growth))
            } else {
                Cmd::none()
            }
        }
        Msg::SetHp(changed_hp) => {
            let mut growth = state.growth.borrow().clone();
            let is_changed = match &mut growth {
                character::Growth::Consumption { hp, .. } => {
                    *hp = changed_hp;
                    true
                }
                _ => false,
            };
            if is_changed {
                Cmd::sub(Sub::SetGrowth(growth))
            } else {
                Cmd::none()
            }
        }
        Msg::SetStatus(status_item) => {
            let mut growth = state.growth.borrow().clone();
            let is_changed = match &mut growth {
                character::Growth::Consumption { status, .. } => {
                    status.set_item(status_item);
                    true
                }
                _ => false,
            };
            if is_changed {
                Cmd::sub(Sub::SetGrowth(growth))
            } else {
                Cmd::none()
            }
        }
        Msg::RemoveSelf => Cmd::sub(Sub::RemoveSelf),
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    match &(*state.growth.borrow()) {
        character::Growth::Acquisition {
            title,
            experience,
            description,
            ..
        } => acquisition(title, *experience, description, state.is_collapsed),
        character::Growth::Consumption {
            title,
            experience,
            class_name,
            hp,
            status,
            description,
            ..
        } => consumption(
            title,
            *experience,
            class_name,
            *hp,
            status,
            description,
            state.is_collapsed,
        ),
    }
}

fn acquisition(title: &String, experience: u32, description: &String, is_collapsed: bool) -> Html {
    Html::div(
        Attributes::new().class("growth").class("growth"),
        Events::new(),
        vec![
            heading(title, experience, true, is_collapsed),
            if !is_collapsed {
                Html::none()
            } else {
                Html::div(
                    Attributes::new()
                        .class("growth__data")
                        .class("growth__data--acquisition"),
                    Events::new(),
                    vec![
                        Html::h4(Attributes::new(), Events::new(), vec![Html::text("メモ")]),
                        Html::textarea(
                            Attributes::new().value(description),
                            Events::new().on_input(Msg::SetDescription),
                            vec![],
                        ),
                    ],
                )
            },
        ],
    )
}

fn consumption(
    title: &String,
    experience: u32,
    class_name: &String,
    hp: i32,
    status: &character::Status,
    description: &String,
    is_collapsed: bool,
) -> Html {
    Html::div(
        Attributes::new().class("growth").class("growth"),
        Events::new(),
        vec![
            heading(title, experience, false, is_collapsed),
            if !is_collapsed {
                Html::none()
            } else {
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
                                Html::div(
                                    Attributes::new().class("growth__list-item-5"),
                                    Events::new(),
                                    vec![Html::text("クラス")],
                                ),
                                Html::div(
                                    Attributes::new().class("growth__list-item-5"),
                                    Events::new(),
                                    vec![Html::component(
                                        select::new()
                                            .with(select::Props {
                                                suid: crate::suid!(),
                                                selected: class_name.clone(),
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
                                            })
                                            .subscribe(|sub| match sub {
                                                select::Sub::ChangeValue(class_name) => {
                                                    Msg::SetClassName(class_name)
                                                }
                                            }),
                                        vec![],
                                    )],
                                ),
                                text::div("HP増加"),
                                Html::input(
                                    Attributes::new().value(hp.to_string()).type_("number"),
                                    Events::new().on_input(move |hp| {
                                        if let Ok(hp) = hp.parse() {
                                            Msg::SetHp(hp)
                                        } else {
                                            Msg::NoOp
                                        }
                                    }),
                                    vec![],
                                ),
                            ],
                        ),
                        Html::div(
                            Attributes::new().class("growth__list"),
                            Events::new(),
                            vec![
                                input_growth_of_status(
                                    "筋力",
                                    status.strength,
                                    character::StatusItem::Strength,
                                ),
                                input_growth_of_status(
                                    "敏捷力",
                                    status.dexterity,
                                    character::StatusItem::Dexterity,
                                ),
                                input_growth_of_status(
                                    "耐久力",
                                    status.constitution,
                                    character::StatusItem::Constitution,
                                ),
                                input_growth_of_status(
                                    "知力",
                                    status.intelligence,
                                    character::StatusItem::Intelligence,
                                ),
                                input_growth_of_status(
                                    "判断力",
                                    status.wisdom,
                                    character::StatusItem::Wisdom,
                                ),
                                input_growth_of_status(
                                    "魅力",
                                    status.charisma,
                                    character::StatusItem::Charisma,
                                ),
                            ]
                            .into_iter()
                            .flatten()
                            .collect(),
                        ),
                        Html::h4(Attributes::new(), Events::new(), vec![Html::text("メモ")]),
                        Html::textarea(
                            Attributes::new().value(description),
                            Events::new().on_input(Msg::SetDescription),
                            vec![],
                        ),
                    ],
                )
            },
        ],
    )
}

fn heading(title: &String, experience: u32, is_acquisition: bool, is_collapsed: bool) -> Html {
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
                if !is_collapsed {
                    icon::arrow_right(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIsCollapsed(true)),
                    )
                } else {
                    icon::arrow_bottom(
                        Attributes::new(),
                        Events::new().on_click(|_| Msg::SetIsCollapsed(false)),
                    )
                },
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
                    Events::new().on_input(Msg::SetTitle),
                    vec![],
                ),
                Html::button(
                    Attributes::new()
                        .class("pure-button")
                        .class("growth__remove-button"),
                    Events::new().on_click(|_| Msg::RemoveSelf),
                    vec![Html::text("×")],
                ),
            ],
        )],
    )
}

fn input_growth_of_status(
    status_name: impl Into<String>,
    value: i32,
    mapper: impl FnOnce(i32) -> character::StatusItem + 'static,
) -> Vec<Html> {
    vec![
        text::div(status_name),
        Html::input(
            Attributes::new().value(value.to_string()).type_("number"),
            Events::new().on_input(move |status_value| {
                if let Ok(status_value) = status_value.parse() {
                    Msg::SetStatus(mapper(status_value))
                } else {
                    Msg::NoOp
                }
            }),
            vec![],
        ),
    ]
}
