use crate::util::prop::C;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct CommonData {
    pub name: String,
}

#[derive(Clone)]
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
