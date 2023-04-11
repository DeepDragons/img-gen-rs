use rand::Rng;
use std::io::{Error, ErrorKind};

use super::rarity::{BACKGROUND, ELEMENTS_LIST};

fn random_number(range: (u8, u8)) -> u8 {
    let (min, max) = range;
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn generate_genes(rarity: usize) -> Result<Vec<u8>, Error> {
    // {rarity}, {background}, {body}, {eyes}, {horns}, {mouth}, {spine?}, {chest?}, {wings?}, {accessories?}, {ears?}, {hair?}, {aura?}, {Backdetails?}
    let mut genes: Vec<u8> = vec![0; 16];
    let elements = match ELEMENTS_LIST.get(rarity) {
        Some(list) => list,
        None => return Err(Error::new(ErrorKind::NotFound, "out of range rarity!")),
    };

    genes[0] = rarity as u8;
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
    genes[13] = random_number(elements.back_details);

    Ok(genes)
}

pub fn genes_view(genes: &Vec<u8>) -> String {
    format!("rarity({}), background({}), body({}), eye({}), horn({}), mouth({}), spine({}), chest({}), wing({}), accessories({}), ears({}), hair({}), aura({}), Backdetails({})",
    genes[0],
    genes[1],
    genes[2],
    genes[3],
    genes[4],
    genes[5],
    genes[6],
    genes[7],
    genes[8],
    genes[9],
    genes[10],
    genes[11],
    genes[12],
    genes[13]
    )
}

#[test]
fn test_generate() {
    let genes = generate_genes(2).unwrap();

    assert!(genes.len() == 16);
}
