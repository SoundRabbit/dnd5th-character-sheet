use crate::util::prop::C;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct CommonData {
    pub name: String,
}

#[derive(Clone)]
pub struct Status {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

pub enum StatusItem {
    Strength(u32),
    Dexterity(u32),
    Constitution(u32),
    Intelligence(u32),
    Wisdom(u32),
    Charisma(u32),
}

pub struct GrowthLog {
    log: Vec<C<Growth>>,
}

#[derive(Clone)]
pub enum Growth {
    Acquisition {
        title: String,
        experience: u32,
        description: String,
    },
    Consumption {
        title: String,
        experience: u32,
        class_name: String,
        status: Status,
        description: String,
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
}

impl GrowthLog {
    pub fn new() -> Self {
        Self { log: Vec::new() }
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
}
