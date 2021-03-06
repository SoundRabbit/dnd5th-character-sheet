use crate::util::prop::C;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct CommonData {
    pub name: String,
    pub initial_status: Status,
    pub race_status: Status,
    pub bonus_status: Status,
    pub bonus_saving: Status,
    pub bonus_initiative: i32,
    pub armor_ac: i32,
    pub shield_ac: i32,
    pub bonus_ac: i32,
    pub basic_mov: i32,
    pub armor_mov: i32,
    pub item_mov: i32,
    pub bonus_mov: i32,
    pub initial_hp: i32,
    pub current_hp: i32,
    pub hitdice: String,
    pub expert_skill: Skill<bool>,
    pub bonus_skill: Skill<i32>,
}

pub enum CommonDataItem {
    Name(String),
    InitialStatus(StatusItem),
    RaceStatus(StatusItem),
    BonusStatus(StatusItem),
    BonusSaving(StatusItem),
    BonusInitiative(i32),
    ArmorAc(i32),
    ShieldAc(i32),
    BonusAc(i32),
    BasicMov(i32),
    ArmorMov(i32),
    ItemMov(i32),
    BonusMov(i32),
    InitialHp(i32),
    CurrentHp(i32),
    HitDice(String),
    ExpertSkill(Skill<bool>),
    BonusSkill(Skill<i32>),
}

#[derive(Clone, Debug)]
pub struct Status {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

pub enum StatusItem {
    Strength(i32),
    Dexterity(i32),
    Constitution(i32),
    Intelligence(i32),
    Wisdom(i32),
    Charisma(i32),
}

#[derive(Debug)]
pub struct Skill<T: Clone> {
    pub acrobatics: T,
    pub animal_handling: T,
    pub arcana: T,
    pub athletics: T,
    pub deception: T,
    pub history: T,
    pub insight: T,
    pub intimidation: T,
    pub investigation: T,
    pub medicine: T,
    pub nature: T,
    pub perception: T,
    pub performance: T,
    pub persuasion: T,
    pub religion: T,
    pub sleight_of_hand: T,
    pub stealth: T,
    pub survival: T,
}

pub struct GrowthLog {
    first_class_name: String,
    log: Vec<C<Growth>>,
}

#[derive(Clone)]
pub enum Growth {
    Acquisition {
        title: String,
        experience: u32,
        description: String,
        key: u64,
    },
    Consumption {
        title: String,
        experience: u32,
        class_name: String,
        hp: i32,
        status: Status,
        description: String,
        key: u64,
    },
}

impl CommonData {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            initial_status: Status::new(),
            race_status: Status::new(),
            bonus_status: Status::new(),
            bonus_saving: Status::new(),
            bonus_initiative: 0,
            armor_ac: 0,
            shield_ac: 0,
            bonus_ac: 0,
            basic_mov: 0,
            armor_mov: 0,
            item_mov: 0,
            bonus_mov: 0,
            initial_hp: 0,
            current_hp: 0,
            hitdice: String::new(),
            expert_skill: Skill::filled(false),
            bonus_skill: Skill::filled(0),
        }
    }

    pub fn set_item(&mut self, item: CommonDataItem) {
        match item {
            CommonDataItem::Name(x) => {
                self.name = x;
            }
            CommonDataItem::InitialStatus(x) => {
                self.initial_status.set_item(x);
            }
            CommonDataItem::RaceStatus(x) => {
                self.race_status.set_item(x);
            }
            CommonDataItem::BonusStatus(x) => {
                self.bonus_status.set_item(x);
            }
            CommonDataItem::BonusSaving(x) => {
                self.bonus_saving.set_item(x);
            }
            CommonDataItem::BonusInitiative(x) => {
                self.bonus_initiative = x;
            }
            CommonDataItem::ArmorAc(x) => {
                self.armor_ac = x;
            }
            CommonDataItem::ShieldAc(x) => {
                self.shield_ac = x;
            }
            CommonDataItem::BonusAc(x) => {
                self.bonus_ac = x;
            }
            CommonDataItem::BasicMov(x) => {
                self.basic_mov = x;
            }
            CommonDataItem::ArmorMov(x) => {
                self.armor_mov = x;
            }
            CommonDataItem::ItemMov(x) => {
                self.item_mov = x;
            }
            CommonDataItem::BonusMov(x) => {
                self.bonus_mov = x;
            }
            CommonDataItem::InitialHp(x) => {
                self.initial_hp = x;
            }
            CommonDataItem::CurrentHp(x) => {
                self.current_hp = x;
            }
            CommonDataItem::HitDice(x) => {
                self.hitdice = x;
            }
            CommonDataItem::ExpertSkill(x) => {
                self.expert_skill = x;
            }
            CommonDataItem::BonusSkill(x) => {
                self.bonus_skill = x;
            }
        }
    }
}

impl Status {
    pub fn new() -> Self {
        Self {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }

    pub fn set_item(&mut self, item: StatusItem) {
        match item {
            StatusItem::Strength(x) => {
                self.strength = x;
            }
            StatusItem::Dexterity(x) => {
                self.dexterity = x;
            }
            StatusItem::Constitution(x) => {
                self.constitution = x;
            }
            StatusItem::Intelligence(x) => {
                self.intelligence = x;
            }
            StatusItem::Wisdom(x) => {
                self.wisdom = x;
            }
            StatusItem::Charisma(x) => {
                self.charisma = x;
            }
        }
    }
}

impl<T: Clone> Skill<T> {
    pub fn filled(v: T) -> Self {
        Self {
            acrobatics: v.clone(),
            animal_handling: v.clone(),
            arcana: v.clone(),
            athletics: v.clone(),
            deception: v.clone(),
            history: v.clone(),
            insight: v.clone(),
            intimidation: v.clone(),
            investigation: v.clone(),
            medicine: v.clone(),
            nature: v.clone(),
            perception: v.clone(),
            performance: v.clone(),
            persuasion: v.clone(),
            religion: v.clone(),
            sleight_of_hand: v.clone(),
            stealth: v.clone(),
            survival: v.clone(),
        }
    }
}

impl GrowthLog {
    pub fn new() -> Self {
        Self {
            first_class_name: String::new(),
            log: Vec::new(),
        }
    }

    pub fn sum_of_acquisition(&self) -> u32 {
        let mut res = 0;
        for growth in &self.log {
            match &(*growth.borrow()) {
                Growth::Acquisition { experience, .. } => {
                    res += *experience;
                }
                _ => {}
            }
        }
        res
    }

    pub fn sum_of_consumption(&self) -> u32 {
        let mut res = 0;
        for growth in &self.log {
            match &(*growth.borrow()) {
                Growth::Consumption { experience, .. } => {
                    res += *experience;
                }
                _ => {}
            }
        }
        res
    }

    pub fn sum_of_experience(&self) -> i64 {
        self.sum_of_acquisition() as i64 - self.sum_of_consumption() as i64
    }

    pub fn class_level(&self) -> Vec<(String, u32)> {
        let mut res = Vec::new();

        if !self.first_class_name.is_empty() {
            res.push((self.first_class_name.clone(), 1));
        }

        for growth in &self.log {
            match &(*growth.borrow()) {
                Growth::Consumption { class_name, .. } => {
                    if !class_name.is_empty() {
                        let mut is_changed = false;
                        for (cn, lv) in &mut res {
                            if cn == class_name {
                                *lv += 1;
                                is_changed = true;
                            }
                        }
                        if !is_changed {
                            res.push((class_name.clone(), 1));
                        }
                    }
                }
                _ => {}
            }
        }

        res
    }

    pub fn status(&self) -> Status {
        let mut res = Status::new();

        for growth in &self.log {
            match &(*growth.borrow()) {
                Growth::Consumption { status, .. } => {
                    res.strength += status.strength;
                    res.dexterity += status.dexterity;
                    res.constitution += status.constitution;
                    res.intelligence += status.intelligence;
                    res.wisdom += status.wisdom;
                    res.charisma += status.charisma;
                }
                _ => {}
            }
        }

        res
    }

    pub fn hp(&self) -> i32 {
        let mut res = 0;

        for growth in &self.log {
            match &(*growth.borrow()) {
                Growth::Consumption { hp, .. } => {
                    res += hp;
                }
                _ => {}
            }
        }

        res
    }

    pub fn first_class_name(&self) -> &String {
        &self.first_class_name
    }

    pub fn set_first_class_name(&mut self, first_class_name: String) {
        self.first_class_name = first_class_name;
    }
}

impl Deref for GrowthLog {
    type Target = Vec<C<Growth>>;
    fn deref(&self) -> &Self::Target {
        &self.log
    }
}

impl DerefMut for GrowthLog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.log
    }
}

impl Growth {
    pub fn is_acquisition(&self) -> bool {
        match self {
            Self::Acquisition { .. } => true,
            _ => false,
        }
    }

    pub fn is_consumption(&self) -> bool {
        match self {
            Self::Consumption { .. } => true,
            _ => false,
        }
    }

    pub fn title(&self) -> &String {
        match self {
            Self::Acquisition { title, .. } => title,
            Self::Consumption { title, .. } => title,
        }
    }

    pub fn experience(&self) -> u32 {
        match self {
            Self::Acquisition { experience, .. } => *experience,
            Self::Consumption { experience, .. } => *experience,
        }
    }

    pub fn description(&self) -> &String {
        match self {
            Self::Acquisition { description, .. } => description,
            Self::Consumption { description, .. } => description,
        }
    }

    pub fn key(&self) -> u64 {
        match self {
            Self::Acquisition { key, .. } => *key,
            Self::Consumption { key, .. } => *key,
        }
    }
}
