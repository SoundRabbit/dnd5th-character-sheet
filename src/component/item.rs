use super::function::empty;
use super::function::text;
use kagura::prelude::*;

pub struct Props {}

pub enum Sub {}

pub type Item = Component<Props, Sub>;

pub fn new() -> Item {
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
        Attributes::new().class("pure-form").class("item"),
        Events::new(),
        vec![
            Html::h2(Attributes::new(), Events::new(), vec![Html::text("所持金")]),
            Html::div(
                Attributes::new().class("item__money-list"),
                Events::new(),
                vec![
                    readonly_money(0, "pp"),
                    readonly_money(0, "gp"),
                    readonly_money(0, "ep"),
                    readonly_money(0, "sp"),
                    readonly_money(0, "cp"),
                    empty::div(),
                    empty::div(),
                    empty::div(),
                    Html::div(
                        Attributes::new().class("item__text-right"),
                        Events::new(),
                        vec![Html::text("合計")],
                    ),
                    Html::div(
                        Attributes::new().class("item__money"),
                        Events::new(),
                        vec![
                            Html::input(
                                Attributes::new().type_("number").flag("readonly"),
                                Events::new(),
                                vec![],
                            ),
                            Html::text("gp"),
                        ],
                    ),
                ],
            ),
            Html::h2(Attributes::new(), Events::new(), vec![Html::text("装備")]),
            Html::div(
                Attributes::new().class("item__item-list"),
                Events::new(),
                vec![],
            ),
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("支出/収入")],
            ),
            Html::div(
                Attributes::new().class("item__expenditure-list"),
                Events::new(),
                vec![],
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

fn readonly_money(value: i64, unit: impl Into<String>) -> Html {
    Html::div(
        Attributes::new().class("item__money"),
        Events::new(),
        vec![readonly_number(value), Html::text(unit)],
    )
}

fn readonly_number(value: i64) -> Html {
    Html::input(
        Attributes::new()
            .type_("number")
            .flag("readonly")
            .value(value.to_string()),
        Events::new(),
        vec![],
    )
}
