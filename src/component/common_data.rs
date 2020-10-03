use super::key_value;
use super::select;
use crate::model::character;
use crate::util::prop::R;
use kagura::prelude::*;

pub struct Props {
    pub common_data: R<character::CommonData>,
    pub growth_log: R<character::GrowthLog>,
}

pub enum Sub {
    SetCommonData(character::CommonDataItem),
}

pub type CommonData = Component<Props, Sub>;

pub fn new() -> CommonData {
    Component::new(init, update, render)
}

struct State {
    common_data: R<character::CommonData>,
    growth_log: R<character::GrowthLog>,
}

enum Msg {
    NoOp,
    SetCommonData(character::CommonDataItem),
}

fn init(_: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    let state = State {
        common_data: props.common_data,
        growth_log: props.growth_log,
    };
    let cmd = Cmd::none();
    let batch = vec![];

    (state, cmd, batch)
}

fn update(_: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::NoOp => Cmd::none(),
        Msg::SetCommonData(common_data_item) => Cmd::sub(Sub::SetCommonData(common_data_item)),
    }
}

fn render(state: &State, _: Vec<Html>) -> Html {
    let common_data = &(*state.common_data.borrow());
    let growth_of_status = state.growth_log.borrow().status();
    let initial_status = &common_data.initial_status;
    let race_status = &common_data.race_status;
    let bonus_status = &common_data.bonus_status;
    let status = character::Status {
        strength: initial_status.strength
            + race_status.strength
            + growth_of_status.strength
            + bonus_status.strength,
        dexterity: initial_status.dexterity
            + race_status.dexterity
            + growth_of_status.dexterity
            + bonus_status.dexterity,
        constitution: initial_status.constitution
            + race_status.constitution
            + growth_of_status.constitution
            + bonus_status.constitution,
        intelligence: initial_status.intelligence
            + race_status.intelligence
            + growth_of_status.intelligence
            + bonus_status.intelligence,
        wisdom: initial_status.wisdom
            + race_status.wisdom
            + growth_of_status.wisdom
            + bonus_status.wisdom,
        charisma: initial_status.charisma
            + race_status.charisma
            + growth_of_status.charisma
            + bonus_status.charisma,
    };
    let bonus = character::Status {
        strength: ((status.strength as f32 - 10.0) / 2.0).floor() as i32,
        dexterity: ((status.dexterity as f32 - 10.0) / 2.0).floor() as i32,
        constitution: ((status.constitution as f32 - 10.0) / 2.0).floor() as i32,
        intelligence: ((status.intelligence as f32 - 10.0) / 2.0).floor() as i32,
        wisdom: ((status.wisdom as f32 - 10.0) / 2.0).floor() as i32,
        charisma: ((status.charisma as f32 - 10.0) / 2.0).floor() as i32,
    };
    let bonus_saving = &common_data.bonus_saving;
    let saving = character::Status {
        strength: bonus.strength + bonus_saving.strength,
        dexterity: bonus.dexterity + bonus_saving.dexterity,
        constitution: bonus.constitution + bonus_saving.constitution,
        intelligence: bonus.intelligence + bonus_saving.intelligence,
        wisdom: bonus.wisdom + bonus_saving.wisdom,
        charisma: bonus.charisma + bonus_saving.charisma,
    };

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
            grid_status(
                &initial_status,
                &race_status,
                &growth_of_status,
                &bonus_status,
                &status,
                &bonus,
            ),
            Html::h2(
                Attributes::new(),
                Events::new(),
                vec![Html::text("セーヴィングスロウ")],
            ),
            grid_saving(&bonus, &bonus_saving, &saving),
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
                            input_status(bonus.dexterity, true, |_| None),
                            Html::text("その他修正"),
                            input_status(common_data.bonus_initiative, false, |x| {
                                Some(character::CommonDataItem::BonusInitiative(x))
                            }),
                            Html::text("イニシアチブ"),
                            input_status(
                                common_data.bonus_initiative + bonus.dexterity,
                                true,
                                |_| None,
                            ),
                        ],
                    ),
                    Html::h2(Attributes::new(), Events::new(), vec![Html::text("AC")]),
                    Html::component(
                        key_value::new().with(key_value::Props {}),
                        vec![
                            Html::text("基本"),
                            input_status(10, true, |_| None),
                            Html::text("能力値修正"),
                            input_status(bonus.dexterity, true, |_| None),
                            Html::text("防具"),
                            input_status(common_data.armor_ac, false, |x| {
                                Some(character::CommonDataItem::ArmorAc(x))
                            }),
                            Html::text("盾"),
                            input_status(common_data.shield_ac, false, |x| {
                                Some(character::CommonDataItem::ShieldAc(x))
                            }),
                            Html::text("その他修正"),
                            input_status(common_data.bonus_ac, false, |x| {
                                Some(character::CommonDataItem::BonusAc(x))
                            }),
                            Html::text("AC"),
                            input_status(
                                common_data.armor_ac + common_data.shield_ac + common_data.bonus_ac,
                                true,
                                |_| None,
                            ),
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
                            input_status(common_data.basic_mov, false, |x| {
                                Some(character::CommonDataItem::BasicMov(x))
                            }),
                            Html::text("防具"),
                            input_status(common_data.armor_mov, false, |x| {
                                Some(character::CommonDataItem::ArmorMov(x))
                            }),
                            Html::text("アイテム"),
                            input_status(common_data.item_mov, false, |x| {
                                Some(character::CommonDataItem::ItemMov(x))
                            }),
                            Html::text("その他修正"),
                            input_status(common_data.bonus_mov, false, |x| {
                                Some(character::CommonDataItem::BonusMov(x))
                            }),
                            Html::text("移動速度"),
                            input_status(
                                common_data.basic_mov
                                    + common_data.armor_mov
                                    + common_data.item_mov
                                    + common_data.bonus_mov,
                                true,
                                |_| None,
                            ),
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
            grid_skill(&bonus, &common_data.expert_skill, &common_data.bonus_skill),
        ],
    )
}

fn grid_status(
    initial_status: &character::Status,
    race_status: &character::Status,
    growth_of_status: &character::Status,
    bonus_status: &character::Status,
    status: &character::Status,
    bonus: &character::Status,
) -> Html {
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
            input_status(initial_status.strength, false, |x| {
                Some(character::CommonDataItem::InitialStatus(
                    character::StatusItem::Strength(x),
                ))
            }),
            input_status(initial_status.dexterity, false, |x| {
                Some(character::CommonDataItem::InitialStatus(
                    character::StatusItem::Dexterity(x),
                ))
            }),
            input_status(initial_status.constitution, false, |x| {
                Some(character::CommonDataItem::InitialStatus(
                    character::StatusItem::Constitution(x),
                ))
            }),
            input_status(initial_status.intelligence, false, |x| {
                Some(character::CommonDataItem::InitialStatus(
                    character::StatusItem::Intelligence(x),
                ))
            }),
            input_status(initial_status.wisdom, false, |x| {
                Some(character::CommonDataItem::InitialStatus(
                    character::StatusItem::Wisdom(x),
                ))
            }),
            input_status(initial_status.charisma, false, |x| {
                Some(character::CommonDataItem::InitialStatus(
                    character::StatusItem::Charisma(x),
                ))
            }),
            text("種族修正"),
            input_status(race_status.strength, false, |x| {
                Some(character::CommonDataItem::RaceStatus(
                    character::StatusItem::Strength(x),
                ))
            }),
            input_status(race_status.dexterity, false, |x| {
                Some(character::CommonDataItem::RaceStatus(
                    character::StatusItem::Dexterity(x),
                ))
            }),
            input_status(race_status.constitution, false, |x| {
                Some(character::CommonDataItem::RaceStatus(
                    character::StatusItem::Constitution(x),
                ))
            }),
            input_status(race_status.intelligence, false, |x| {
                Some(character::CommonDataItem::RaceStatus(
                    character::StatusItem::Intelligence(x),
                ))
            }),
            input_status(race_status.wisdom, false, |x| {
                Some(character::CommonDataItem::RaceStatus(
                    character::StatusItem::Wisdom(x),
                ))
            }),
            input_status(race_status.charisma, false, |x| {
                Some(character::CommonDataItem::RaceStatus(
                    character::StatusItem::Charisma(x),
                ))
            }),
            text("成長"),
            input_status(growth_of_status.strength, true, |_| None),
            input_status(growth_of_status.dexterity, true, |_| None),
            input_status(growth_of_status.constitution, true, |_| None),
            input_status(growth_of_status.intelligence, true, |_| None),
            input_status(growth_of_status.wisdom, true, |_| None),
            input_status(growth_of_status.charisma, true, |_| None),
            text("その他修正"),
            input_status(bonus_status.strength, false, |x| {
                Some(character::CommonDataItem::BonusStatus(
                    character::StatusItem::Strength(x),
                ))
            }),
            input_status(bonus_status.dexterity, false, |x| {
                Some(character::CommonDataItem::BonusStatus(
                    character::StatusItem::Dexterity(x),
                ))
            }),
            input_status(bonus_status.constitution, false, |x| {
                Some(character::CommonDataItem::BonusStatus(
                    character::StatusItem::Constitution(x),
                ))
            }),
            input_status(bonus_status.intelligence, false, |x| {
                Some(character::CommonDataItem::BonusStatus(
                    character::StatusItem::Intelligence(x),
                ))
            }),
            input_status(bonus_status.wisdom, false, |x| {
                Some(character::CommonDataItem::BonusStatus(
                    character::StatusItem::Wisdom(x),
                ))
            }),
            input_status(bonus_status.charisma, false, |x| {
                Some(character::CommonDataItem::BonusStatus(
                    character::StatusItem::Charisma(x),
                ))
            }),
            text("能力値"),
            input_status(status.strength, true, |_| None),
            input_status(status.dexterity, true, |_| None),
            input_status(status.constitution, true, |_| None),
            input_status(status.intelligence, true, |_| None),
            input_status(status.wisdom, true, |_| None),
            input_status(status.charisma, true, |_| None),
            text("能力値修正"),
            input_status(bonus.strength, true, |_| None),
            input_status(bonus.dexterity, true, |_| None),
            input_status(bonus.constitution, true, |_| None),
            input_status(bonus.intelligence, true, |_| None),
            input_status(bonus.wisdom, true, |_| None),
            input_status(bonus.charisma, true, |_| None),
        ],
    )
}

fn text(text: impl Into<String>) -> Html {
    Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
}

fn input_status(
    value: i32,
    is_readonly: bool,
    on_input: impl FnOnce(i32) -> Option<character::CommonDataItem> + 'static,
) -> Html {
    let attrs = if is_readonly {
        Attributes::new().flag("readonly")
    } else {
        Attributes::new()
    };
    Html::input(
        attrs.type_("number").value(value.to_string()),
        Events::new().on_input(move |x| {
            x.parse()
                .ok()
                .and_then(move |x| on_input(x))
                .map(|x| Msg::SetCommonData(x))
                .unwrap_or(Msg::NoOp)
        }),
        vec![],
    )
}

fn grid_saving(
    bonus: &character::Status,
    bonus_saving: &character::Status,
    saving: &character::Status,
) -> Html {
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
            input_status(bonus.strength, true, |_| None),
            input_status(bonus.dexterity, true, |_| None),
            input_status(bonus.constitution, true, |_| None),
            input_status(bonus.intelligence, true, |_| None),
            input_status(bonus.wisdom, true, |_| None),
            input_status(bonus.charisma, true, |_| None),
            text("習熟"),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            input_skill_expert(false),
            text("その他修正"),
            input_status(bonus_saving.strength, false, |x| {
                Some(character::CommonDataItem::BonusSaving(
                    character::StatusItem::Strength(x),
                ))
            }),
            input_status(bonus_saving.dexterity, false, |x| {
                Some(character::CommonDataItem::BonusSaving(
                    character::StatusItem::Dexterity(x),
                ))
            }),
            input_status(bonus_saving.constitution, false, |x| {
                Some(character::CommonDataItem::BonusSaving(
                    character::StatusItem::Constitution(x),
                ))
            }),
            input_status(bonus_saving.intelligence, false, |x| {
                Some(character::CommonDataItem::BonusSaving(
                    character::StatusItem::Intelligence(x),
                ))
            }),
            input_status(bonus_saving.wisdom, false, |x| {
                Some(character::CommonDataItem::BonusSaving(
                    character::StatusItem::Wisdom(x),
                ))
            }),
            input_status(bonus_saving.charisma, false, |x| {
                Some(character::CommonDataItem::BonusSaving(
                    character::StatusItem::Charisma(x),
                ))
            }),
            text("セーヴ"),
            input_status(saving.strength, true, |_| None),
            input_status(saving.dexterity, true, |_| None),
            input_status(saving.constitution, true, |_| None),
            input_status(saving.intelligence, true, |_| None),
            input_status(saving.wisdom, true, |_| None),
            input_status(saving.charisma, true, |_| None),
        ],
    )
}

fn grid_skill(
    bonus: &character::Status,
    expert_skill: &character::Skill<bool>,
    bonus_skill: &character::Skill<i32>,
) -> Html {
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
            input_skill_status("魅", bonus.charisma),
            input_skill_expert(expert_skill.intimidation),
            input_skill(0, false),
            input_skill(0, true),
            text("〈医術〉"),
            input_skill_status("判", bonus.wisdom),
            input_skill_expert(expert_skill.medicine),
            input_skill(0, false),
            input_skill(0, true),
            text("〈運動〉"),
            input_skill_status("筋", bonus.strength),
            input_skill_expert(expert_skill.athletics),
            input_skill(0, false),
            input_skill(0, true),
            text("〈隠密〉"),
            input_skill_status("敏", bonus.dexterity),
            input_skill_expert(expert_skill.stealth),
            input_skill(0, false),
            input_skill(0, true),
            text("〈軽業〉"),
            input_skill_status("敏", bonus.dexterity),
            input_skill_expert(expert_skill.acrobatics),
            input_skill(0, false),
            input_skill(0, true),
            text("〈看破〉"),
            input_skill_status("判", bonus.wisdom),
            input_skill_expert(expert_skill.insight),
            input_skill(0, false),
            input_skill(0, true),
            text("〈芸能〉"),
            input_skill_status("魅", bonus.charisma),
            input_skill_expert(expert_skill.performance),
            input_skill(0, false),
            input_skill(0, true),
            text("〈自然〉"),
            input_skill_status("知", bonus.intelligence),
            input_skill_expert(expert_skill.nature),
            input_skill(0, false),
            input_skill(0, true),
            text("〈宗教〉"),
            input_skill_status("知", bonus.intelligence),
            input_skill_expert(expert_skill.religion),
            input_skill(0, false),
            input_skill(0, true),
            text("〈生存〉"),
            input_skill_status("判", bonus.wisdom),
            input_skill_expert(expert_skill.survival),
            input_skill(0, false),
            input_skill(0, true),
            text("〈説得〉"),
            input_skill_status("魅", bonus.charisma),
            input_skill_expert(expert_skill.persuasion),
            input_skill(0, false),
            input_skill(0, true),
            text("〈捜査〉"),
            input_skill_status("知", bonus.intelligence),
            input_skill_expert(expert_skill.investigation),
            input_skill(0, false),
            input_skill(0, true),
            text("〈知覚〉"),
            input_skill_status("判", bonus.wisdom),
            input_skill_expert(expert_skill.perception),
            input_skill(0, false),
            input_skill(0, true),
            text("〈手先の早業〉"),
            input_skill_status("敏", bonus.dexterity),
            input_skill_expert(expert_skill.sleight_of_hand),
            input_skill(0, false),
            input_skill(0, true),
            text("〈動物使い〉"),
            input_skill_status("判", bonus.wisdom),
            input_skill_expert(expert_skill.animal_handling),
            input_skill(0, false),
            input_skill(0, true),
            text("〈ペテン〉"),
            input_skill_status("魅", bonus.charisma),
            input_skill_expert(expert_skill.deception),
            input_skill(0, false),
            input_skill(0, true),
            text("〈魔法学〉"),
            input_skill_status("知", bonus.intelligence),
            input_skill_expert(expert_skill.arcana),
            input_skill(0, false),
            input_skill(0, true),
            text("〈歴史〉"),
            input_skill_status("知", bonus.intelligence),
            input_skill_expert(expert_skill.history),
            input_skill(0, false),
            input_skill(0, true),
        ],
    )
}

fn input_skill_status(name: &str, value: i32) -> Html {
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

fn input_skill(value: i32, is_readonly: bool) -> Html {
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
