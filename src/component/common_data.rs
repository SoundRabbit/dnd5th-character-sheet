use super::key_value;
use super::select;
use crate::model::character;
use crate::util::prop::R;
use kagura::prelude::*;

pub struct Props {
    pub common_data: R<character::CommonData>,
}

pub enum Sub {
    SetCharacterName(String),
}

pub type CommonData = Component<Props, Sub>;

pub fn new() -> CommonData {
    Component::new(init, update, render)
}

struct State {
    common_data: R<character::CommonData>,
}

enum Msg {}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        common_data: props.common_data,
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State, _: Vec<Html>) -> Html {
    let common_data = &(*state.common_data.borrow());

    Html::form(
        Attributes::new().class("pure-form").class("common-data"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("common-data__name"),
                Events::new(),
                vec![
                    Html::text("キャラクター名"),
                    Html::input(
                        Attributes::new().value(&common_data.name),
                        Events::new(),
                        vec![],
                    ),
                    Html::text("プレイヤー名"),
                    Html::input(Attributes::new(), Events::new(), vec![]),
                ],
            ),
            Html::div(
                Attributes::new().class("common-data__list"),
                Events::new(),
                vec![
                    Html::text("種族"),
                    Html::component(
                        select::new().with(select::Props {
                            suid: crate::suid!(),
                            selected: String::new(),
                            option: vec![String::from("エルフ")],
                        }),
                        vec![],
                    ),
                    Html::text("サイズ"),
                    Html::component(
                        select::new().with(select::Props {
                            suid: crate::suid!(),
                            selected: String::new(),
                            option: vec![String::from("エルフ")],
                        }),
                        vec![],
                    ),
                    Html::text("属性"),
                    Html::component(
                        select::new().with(select::Props {
                            suid: crate::suid!(),
                            selected: String::new(),
                            option: vec![String::from("エルフ")],
                        }),
                        vec![],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("common-data__list"),
                Events::new(),
                vec![
                    Html::text("その他メモ"),
                    Html::textarea(Attributes::new().nut("rows", 5), Events::new(), vec![]),
                ],
            ),
            Html::h2(Attributes::new(), Events::new(), vec![Html::text("能力値")]),
            grid_status(),
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("セーヴィングスロウ")],
            ),
            grid_saving(),
            Html::div(
                Attributes::new().class("common-data__list"),
                Events::new(),
                vec![
                    Html::h2(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("イニシアチブ")],
                    ),
                    Html::component(
                        key_value::new().with(key_value::Props {}),
                        vec![
                            Html::text("能力値修正"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("その他修正"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("イニシアチブ"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                        ],
                    ),
                    Html::h2(Attributes::new(), Events::new(), vec![Html::text("AC")]),
                    Html::component(
                        key_value::new().with(key_value::Props {}),
                        vec![
                            Html::text("基本"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("能力値修正"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("防具"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("盾"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("その他修正"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("AC"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                        ],
                    ),
                ],
            ),
            Html::div(
                Attributes::new().class("common-data__list"),
                Events::new(),
                vec![
                    Html::h2(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("移動速度")],
                    ),
                    Html::component(
                        key_value::new().with(key_value::Props {}),
                        vec![
                            Html::text("基本"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("防具"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("アイテム"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("その他修正"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("移動速度"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                        ],
                    ),
                    Html::h2(
                        Attributes::new(),
                        Events::new(),
                        vec![Html::text("ヒットポイント")],
                    ),
                    Html::component(
                        key_value::new().with(key_value::Props {}),
                        vec![
                            Html::text("Lv1時"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("成長"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("最大HP"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("現在HP"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                            Html::text("ヒットダイス"),
                            Html::input(Attributes::new(), Events::new(), vec![]),
                        ],
                    ),
                ],
            ),
            Html::h2(Attributes::new(), Events::new(), vec![Html::text("技能")]),
            grid_skill(),
        ],
    )
}

fn grid_status() -> Html {
    Html::div(
        Attributes::new().class("common-data__status-grid"),
        Events::new(),
        vec![
            text(""),
            text("筋力"),
            text("敏捷力"),
            text("耐久力"),
            text("知力"),
            text("判断力"),
            text("魅力"),
            text("初期値"),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            text("種族修正"),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            text("成長"),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            text("その他修正"),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            text("能力値"),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            text("能力値修正"),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
        ],
    )
}

fn text(text: impl Into<String>) -> Html {
    Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
}

fn input_status(value: u32, is_readonly: bool) -> Html {
    let attrs = if is_readonly {
        Attributes::new().flag("readonly")
    } else {
        Attributes::new()
    };
    Html::input(
        attrs.type_("number").value(value.to_string()),
        Events::new(),
        vec![],
    )
}

fn grid_saving() -> Html {
    Html::div(
        Attributes::new().class("common-data__status-grid"),
        Events::new(),
        vec![
            text(""),
            text("筋力"),
            text("敏捷力"),
            text("耐久力"),
            text("知力"),
            text("判断力"),
            text("魅力"),
            text("能力値修正"),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            text("習熟"),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            text("その他修正"),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            input_status(0, false),
            text("セーヴ"),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
            input_status(0, true),
        ],
    )
}

fn grid_skill() -> Html {
    Html::div(
        Attributes::new().class("common-data__skill-grid"),
        Events::new(),
        vec![
            text(""),
            text("能力"),
            text("習熟"),
            text("その他修正"),
            text("技能値"),
            text("〈威圧〉"),
            input_skill_status("魅", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈医術〉"),
            input_skill_status("判", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈運動〉"),
            input_skill_status("筋", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈隠密〉"),
            input_skill_status("敏", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈軽業〉"),
            input_skill_status("敏", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈看破〉"),
            input_skill_status("判", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈芸能〉"),
            input_skill_status("魅", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈自然〉"),
            input_skill_status("知", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈宗教〉"),
            input_skill_status("知", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈生存〉"),
            input_skill_status("判", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈説得〉"),
            input_skill_status("魅", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈捜査〉"),
            input_skill_status("知", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈知覚〉"),
            input_skill_status("判", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈手先の早業〉"),
            input_skill_status("敏", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈動物使い〉"),
            input_skill_status("判", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈ペテン〉"),
            input_skill_status("魅", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈魔法学〉"),
            input_skill_status("知", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
            text("〈歴史〉"),
            input_skill_status("知", 0),
            input_skill_expert(false),
            input_skill(0, false),
            input_skill(0, true),
        ],
    )
}

fn input_skill_status(name: &str, value: u32) -> Html {
    Html::input(
        Attributes::new()
            .flag("readonly")
            .type_("text")
            .value(String::new() + "【" + name + "】" + &value.to_string()),
        Events::new(),
        vec![],
    )
}

fn input_skill_expert(is_checked: bool) -> Html {
    let attrs = if is_checked {
        Attributes::new().checked()
    } else {
        Attributes::new()
    };
    Html::input(attrs.type_("checkbox"), Events::new(), vec![])
}

fn input_skill(value: u32, is_readonly: bool) -> Html {
    let attrs = if is_readonly {
        Attributes::new().flag("readonly")
    } else {
        Attributes::new()
    };
    Html::input(
        attrs.type_("number").value(value.to_string()),
        Events::new(),
        vec![],
    )
}
