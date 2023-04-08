use rand::Rng;

use super::rarity::{BACKGROUND, ELEMENTS_LIST};

fn random_number(range: (u8, u8)) -> u8 {
    let (min, max) = range;
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn generate_genes(rarity: u8) -> Vec<u8> {
    // {rarity}, {background}, {body}, {eyes}, {horns}, {mouth}, {spine?}, {chest?}, {wings?}, {accessories?}, {ears?}, {hair?}, {aura?}
    let mut genes: Vec<u8> = Vec::with_capacity(16);
    let elements = ELEMENTS_LIST
        .get(rarity as usize)
        .expect("Out of range rarity number!");

    genes[0] = rarity;
    genes[1] = random_number(BACKGROUND);

    genes[2] = random_number(elements.body);
    genes[3] = random_number(elements.eyes);
    genes[4] = random_number(elements.horns);
    genes[5] = random_number(elements.mouth);
    genes[6] = random_number(elements.spine);
    genes[7] = random_number(elements.chest);
    genes[8] = random_number(elements.wings);
    genes[9] = random_number(elements.accessories);
    genes[10] = random_number(elements.ears);
    genes[11] = random_number(elements.hair);
    genes[12] = random_number(elements.aura);

    genes
}
