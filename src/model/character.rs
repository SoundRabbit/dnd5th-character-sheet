use crate::util::prop::C;

pub struct CommonData {
    pub name: String,
}

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
