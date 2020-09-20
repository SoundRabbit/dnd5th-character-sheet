use super::common_data;
use super::experience;
use super::function::text;
use crate::model::{character, Character};
use crate::util::prop::C;
use kagura::prelude::*;

pub struct Props {}

pub enum Sub {}

pub fn new() -> Component<Props, Sub> {
    Component::new(init, update, render)
}

struct State {
    character: C<Character>,
}

enum Msg {}

fn init(_: Option<State>, _: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        character: C::new(Character::new()),
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(_: &mut State, _: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State, _: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("app"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new().class("app__left-menu"),
                Events::new(),
                vec![],
            ),
            Html::div(
                Attributes::new().class("app__left"),
                Events::new(),
                vec![Html::component(
                    common_data::new().with(common_data::Props {
                        character: state.character.r(),
                    }),
                    vec![],
                )],
            ),
            Html::div(
                Attributes::new().class("app__scroll"),
                Events::new(),
                vec![
                    right_menu(text::div("経験点/成長"), text::div("0点")),
                    Html::div(
                        Attributes::new().class("app__right"),
                        Events::new(),
                        vec![Html::component(
                            experience::new().with(experience::Props {}),
                            vec![],
                        )],
                    ),
                    right_menu(text::div("アイテム/所持金"), text::div("0gp")),
                    Html::div(Attributes::new().class("app__right"), Events::new(), vec![]),
                    right_menu(text::div("ウィザード"), text::div("1Lv")),
                    Html::div(Attributes::new().class("app__right"), Events::new(), vec![]),
                ],
            ),
        ],
    )
}

fn right_menu(right: Html, left: Html) -> Html {
    Html::div(
        Attributes::new().class("app__right-menu"),
        Events::new(),
        vec![right, left],
    )
}
