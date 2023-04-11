use rand::Rng;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use super::rarity::{BACKGROUND, ELEMENTS_LIST};

const N100: f64 = 100.0;

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

fn generate_genes_list(rarity: u8, curve: f64) -> Vec<Vec<u8>> {
    vec![rarity; curve as usize]
        .iter()
        .map(|el| generate_genes(*el as usize).unwrap())
        .collect()
}

pub fn generate_parts(
    parts: (u8, u8, u8, u8, u8, u8, u8),
    total: usize,
) -> HashMap<u8, Vec<Vec<u8>>> {
    let mut map: HashMap<u8, Vec<Vec<u8>>> = HashMap::new();
    let total_elements = total as f64;
    let zero_count = (parts.0 as f64 / N100) * total_elements;
    let one_count = (parts.1 as f64 / N100) * total_elements;
    let two_count = (parts.2 as f64 / N100) * total_elements;
    let three_count = (parts.3 as f64 / N100) * total_elements;
    let four_count = (parts.4 as f64 / N100) * total_elements;
    let five_count = (parts.5 as f64 / N100) * total_elements;
    let six_count = (parts.6 as f64) * total_elements;

    map.insert(0, generate_genes_list(0, zero_count));
    map.insert(1, generate_genes_list(1, one_count));
    map.insert(2, generate_genes_list(2, two_count));
    map.insert(3, generate_genes_list(3, three_count));
    map.insert(4, generate_genes_list(4, four_count));
    map.insert(5, generate_genes_list(5, five_count));
    map.insert(6, generate_genes_list(6, six_count));

    map
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
