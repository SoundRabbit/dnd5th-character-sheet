use super::common_data;
use super::function::text;
use super::item;
use super::{experience, Exprerience};
use crate::model::character;
use crate::util::prop::C;
use kagura::prelude::*;

pub struct Props {}

pub enum Sub {}

pub fn new() -> Component<Props, Sub> {
    Component::new(init, update, render)
}

struct State {
    common_data: C<character::CommonData>,
    growth_log: C<character::GrowthLog>,
    experience: Exprerience,
}

enum Msg {
    AddGrowth(character::Growth),
    SetGrowth(usize, character::Growth),
    SetFirstClassName(String),
    RemoveGrowth(usize),
}

fn init(_: Option<State>, _: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        common_data: C::new(character::CommonData::new()),
        growth_log: C::new(character::GrowthLog::new()),
        experience: experience::new(),
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::AddGrowth(growth) => {
            state.growth_log.borrow_mut().push(C::new(growth));
            Cmd::none()
        }
        Msg::SetGrowth(i, growth) => {
            state.growth_log.borrow_mut()[i] = C::new(growth);
            Cmd::none()
        }
        Msg::SetFirstClassName(class_name) => {
            state
                .growth_log
                .borrow_mut()
                .set_first_class_name(class_name);
            Cmd::none()
        }
        Msg::RemoveGrowth(i) => {
            state.growth_log.borrow_mut().remove(i);
            Cmd::none()
        }
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    Html::div(
        Attributes::new().class("app"),
        Events::new(),
        vec![
            Html::div(
                Attributes::new()
                    .class("app__column")
                    .class("app__left-menu"),
                Events::new(),
                vec![],
            ),
            Html::div(
                Attributes::new().class("app__column").class("app__left"),
                Events::new(),
                vec![Html::component(
                    common_data::new().with(common_data::Props {
                        common_data: state.common_data.r(),
                    }),
                    vec![],
                )],
            ),
            Html::div(
                Attributes::new().class("app__scroll"),
                Events::new(),
                right_contents(state),
            ),
        ],
    )
}

fn right_contents(state: &State) -> Vec<Html> {
    let mut content = vec![
        right_menu(text::div("経験点/成長"), text::div("0点")),
        Html::div(
            Attributes::new().class("app__column").class("app__right"),
            Events::new(),
            vec![Html::component(
                state
                    .experience
                    .with(experience::Props {
                        growth_log: state.growth_log.r(),
                    })
                    .subscribe(|sub| match sub {
                        experience::Sub::AddGrowth(growth) => Msg::AddGrowth(growth),
                        experience::Sub::SetGrowth(i, growth) => Msg::SetGrowth(i, growth),
                        experience::Sub::SetFirstClassName(class_name) => {
                            Msg::SetFirstClassName(class_name)
                        }
                        experience::Sub::RemoveGrowth(i) => Msg::RemoveGrowth(i),
                    }),
                vec![],
            )],
        ),
        right_menu(text::div("所持金/装備"), text::div("0gp")),
        Html::div(
            Attributes::new().class("app__column").class("app__right"),
            Events::new(),
            vec![Html::component(item::new().with(item::Props {}), vec![])],
        ),
    ];

    let classes = state.growth_log.borrow().class_level();

    for (class_name, class_level) in classes {
        content.push(right_menu(
            text::div(class_name),
            text::div(class_level.to_string() + "Lv"),
        ));
        content.push(Html::div(
            Attributes::new().class("app__column").class("app__right"),
            Events::new(),
            vec![],
        ));
    }

    content
}

fn right_menu(right: Html, left: Html) -> Html {
    Html::div(
        Attributes::new()
            .class("app__column")
            .class("app__right-menu"),
        Events::new(),
        vec![right, left],
    )
}
