use rand::Rng;
use std::io::{Error, ErrorKind};

use super::rarity::{BACKGROUND, ELEMENTS_LIST};

fn random_number(range: (u8, u8)) -> u8 {
    let (min, max) = range;
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn generate_genes(rarity: u8) -> Result<Vec<u8>, Error> {
    // {rarity}, {background}, {body}, {eyes}, {horns}, {mouth}, {spine?}, {chest?}, {wings?}, {accessories?}, {ears?}, {hair?}, {aura?}
    let mut genes: Vec<u8> = Vec::with_capacity(16);
    let elements = match ELEMENTS_LIST.get(rarity as usize) {
        Some(list) => list,
        None => return Err(Error::new(ErrorKind::NotFound, "out of range rarity!")),
    };

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

    Ok(genes)
}
