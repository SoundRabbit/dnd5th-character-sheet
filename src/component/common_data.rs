use super::select;
use crate::model::Character;
use crate::util::prop::R;
use kagura::prelude::*;

pub struct Props {
    pub character: R<Character>,
}

pub enum Sub {
    SetCharacterName(String),
}

pub type CommonData = Component<Props, Sub>;

pub fn new() -> CommonData {
    Component::new(init, update, render)
}

struct State {
    character: R<Character>,
}

enum Msg {}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        character: props.character,
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State, _: Vec<Html>) -> Html {
    let character: &Character = &(*state.character.borrow());

    Html::form(
        Attributes::new().class("pure-form").class("common-data"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("common-data__name-and-race"),
                Events::new(),
                vec![
                    Html::text("キャラクター名"),
                    Html::input(
                        Attributes::new().value(&character.name),
                        Events::new(),
                        vec![],
                    ),
                    Html::text("種族"),
                    Html::component(
                        select::new().with(select::Props {
                            option: vec![String::from("エルフ")],
                        }),
                        vec![],
                    ),
                ],
            ),
            Html::h2(Attributes::new(), Events::new(), vec![Html::text("能力値")]),
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
            ),
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
