pub struct Rarity {
    pub accessories: (u8, u8),
    pub aura: (u8, u8),
    pub body: (u8, u8),
    pub chest: (u8, u8),
    pub eyes: (u8, u8),
    pub hair: (u8, u8),
    pub horns: (u8, u8),
    pub mouth: (u8, u8),
    pub spine: (u8, u8),
    pub wings: (u8, u8),
    pub ears: (u8, u8),
}

pub const BACKGROUND: (u8, u8) = (0, 9);
pub const RARITY: (u8, u8) = (0, 6);
pub const ELEMENTS_LIST: [Rarity; 7] = [
    Rarity {
        accessories: (0, 0),
        aura: (0, 0),
        body: (0, 9),
        chest: (0, 0),
        eyes: (0, 9),
        hair: (0, 0),
        horns: (0, 9),
        mouth: (0, 9),
        spine: (0, 0),
        wings: (0, 0),
        ears: (0, 0),
    },
    Rarity {
        accessories: (0, 0),
        aura: (0, 0),
        body: (0, 5),
        chest: (0, 0),
        eyes: (0, 5),
        hair: (0, 0),
        horns: (0, 5),
        mouth: (0, 5),
        spine: (0, 6),
        wings: (0, 0),
        ears: (0, 0),
    },
    Rarity {
        accessories: (0, 0),
        aura: (0, 0),
        body: (0, 3),
        chest: (0, 4),
        eyes: (0, 3),
        hair: (0, 0),
        horns: (0, 3),
        mouth: (0, 3),
        spine: (0, 4),
        wings: (0, 0),
        ears: (0, 0),
    },
    Rarity {
        accessories: (0, 0),
        aura: (0, 0),
        body: (0, 3),
        chest: (0, 4),
        eyes: (0, 3),
        hair: (0, 0),
        horns: (0, 3),
        mouth: (0, 3),
        spine: (0, 4),
        wings: (0, 4),
        ears: (0, 0),
    },
    Rarity {
        accessories: (0, 0),
        aura: (0, 3),
        body: (0, 2),
        chest: (0, 3),
        eyes: (0, 2),
        hair: (0, 0),
        horns: (0, 2),
        mouth: (0, 2),
        spine: (0, 3),
        wings: (0, 3),
        ears: (0, 0),
    },
    Rarity {
        accessories: (0, 0),
        aura: (0, 3),
        body: (0, 2),
        chest: (0, 3),
        eyes: (0, 3),
        hair: (0, 0),
        horns: (0, 2),
        mouth: (0, 2),
        spine: (0, 3),
        wings: (0, 3),
        ears: (0, 0),
    },
    Rarity {
        accessories: (0, 1),
        aura: (0, 0),
        body: (0, 1),
        chest: (0, 2),
        eyes: (0, 1),
        hair: (0, 2),
        horns: (0, 1),
        mouth: (0, 1),
        spine: (0, 2),
        wings: (0, 3),
        ears: (1, 2),
    },
];
