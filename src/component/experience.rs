use super::function::empty;
use super::function::text;
use super::key_value;
use super::select;
use super::{growth, Growth};
use crate::model::character;
use crate::util::prop::R;
use kagura::prelude::*;

pub struct Props {
    pub growth_log: R<character::GrowthLog>,
}

pub enum Sub {
    AddGrowth(character::Growth),
    SetGrowth(usize, character::Growth),
    SetFirstClassName(String),
}

pub type Exprerience = Component<Props, Sub>;

pub fn new() -> Exprerience {
    Component::new(init, update, render)
}

struct State {
    growth_log: R<character::GrowthLog>,
    growthes: Vec<Growth>,
}

enum Msg {
    AddGrowth,
    SetGrowth(usize, character::Growth),
    SetFirstClassName(String),
}

fn init(_: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        growth_log: props.growth_log.clone(),
        growthes: props
            .growth_log
            .borrow()
            .iter()
            .map(|_| growth::new())
            .collect(),
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(_: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::AddGrowth => {
            let growth = character::Growth::Acquisition {
                title: String::new(),
                experience: 0,
                description: String::new(),
            };
            Cmd::sub(Sub::AddGrowth(growth))
        }
        Msg::SetGrowth(i, growth) => Cmd::sub(Sub::SetGrowth(i, growth)),
        Msg::SetFirstClassName(class_name) => Cmd::sub(Sub::SetFirstClassName(class_name)),
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("pure-form").class("experience"),
        Events::new(),
        vec![
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("習得済みクラス")],
            ),
            Html::component(
                key_value::new().with(key_value::Props {}),
                state
                    .growth_log
                    .borrow()
                    .class_level()
                    .into_iter()
                    .map(|(class_name, class_level)| {
                        vec![
                            text::div(class_name),
                            text::div(class_level.to_string() + "Lv"),
                        ]
                    })
                    .flatten()
                    .collect(),
            ),
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("経験点の獲得/消費")],
            ),
            Html::div(
                Attributes::new().class("experience__sum"),
                Events::new(),
                vec![
                    text::div("獲得経験点"),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .flag("readonly")
                            .value(state.growth_log.borrow().sum_of_acquisition().to_string()),
                        Events::new(),
                        vec![],
                    ),
                    text::div("消費経験点"),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .flag("readonly")
                            .value(state.growth_log.borrow().sum_of_consumption().to_string()),
                        Events::new(),
                        vec![],
                    ),
                    text::div("所持経験点"),
                    Html::input(
                        Attributes::new()
                            .type_("number")
                            .flag("readonly")
                            .value(state.growth_log.borrow().sum_of_experience().to_string()),
                        Events::new(),
                        vec![],
                    ),
                ],
            ),
            Html::component(
                key_value::new().with(key_value::Props {}),
                vec![
                    Html::text("初期クラス"),
                    Html::component(
                        select::new()
                            .with(select::Props {
                                suid: crate::suid!(),
                                selected: state.growth_log.borrow().first_class_name().clone(),
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
                                    Msg::SetFirstClassName(class_name)
                                }
                            }),
                        vec![],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("experience__growth-log"),
                Events::new(),
                state
                    .growth_log
                    .borrow()
                    .iter()
                    .zip(state.growthes.iter())
                    .enumerate()
                    .map(|(i, (g, c))| {
                        Html::component(
                            c.with(growth::Props { growth: g.r() }).subscribe(
                                move |sub| match sub {
                                    growth::Sub::SetGrowth(growth) => Msg::SetGrowth(i, growth),
                                },
                            ),
                            vec![],
                        )
                    })
                    .collect(),
            ),
            Html::button(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-primary"),
                Events::new().on_click(|_| Msg::AddGrowth),
                vec![Html::text("追加")],
            ),
        ],
    )
}
