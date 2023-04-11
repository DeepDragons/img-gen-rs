use rand::Rng;
use std::io::{Error, ErrorKind};

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

pub fn generate_pars(parts: (u8, u8, u8, u8, u8, u8, u8), total: usize) -> Vec<u8> {
    let total_elements = total as f64;
    let mut array: Vec<u8> = vec![];
    let zero_count = (parts.0 as f64 / N100) * total_elements;
    let one_count = (parts.1 as f64 / N100) * total_elements;
    let two_count = (parts.2 as f64 / N100) * total_elements;
    let three_count = (parts.3 as f64 / N100) * total_elements;
    let four_count = (parts.4 as f64 / N100) * total_elements;
    let five_count = (parts.5 as f64 / N100) * total_elements;
    let six_count = (parts.6 as f64) * total_elements;

    array.extend(vec![0; zero_count as usize]);
    array.extend(vec![1; one_count as usize]);
    array.extend(vec![2; two_count as usize]);
    array.extend(vec![3; three_count as usize]);
    array.extend(vec![4; four_count as usize]);
    array.extend(vec![5; five_count as usize]);
    array.extend(vec![6; six_count as usize]);

    array
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
