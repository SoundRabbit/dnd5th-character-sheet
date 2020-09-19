use super::common_data;
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

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        character: C::new(Character::new()),
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State, children: Vec<Html>) -> Html {
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
                Attributes::new().class("app__right-menu"),
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
            Html::div(Attributes::new().class("app__right"), Events::new(), vec![]),
        ],
    )
}
