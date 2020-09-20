use super::key_value;
use super::select;
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

fn render(_: &State, _: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("pure-form").class("experience"),
        Events::new(),
        vec![
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("習得済みクラス")],
            ),
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("経験点の獲得/消費")],
            ),
            Html::component(
                key_value::new().with(key_value::Props {}),
                vec![
                    Html::text("初期クラス"),
                    Html::component(
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
                    ),
                ],
            ),
            Html::button(
                Attributes::new()
                    .class("pure-button")
                    .class("pure-button-primary"),
                Events::new(),
                vec![Html::text("追加")],
            ),
        ],
    )
}
