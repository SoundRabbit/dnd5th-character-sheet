use super::key_value;
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

    Html::div(
        Attributes::new().class("pure-form").class("common-data"),
        Events::new(),
        vec![Html::component(
            key_value::new().with(key_value::Props {}),
            vec![
                Html::text("キャラクター名"),
                Html::input(
                    Attributes::new().value(&character.name),
                    Events::new(),
                    vec![],
                ),
            ],
        )],
    )
}
