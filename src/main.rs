use img_gen_rs::{compositor::generate::Composite, genes::generate::generate_genes};

fn main() {
    let rarity = 0;
    let data = String::from("/home/rinat/source/dragon_imgs");
    let tmp = String::from("/home/rinat/source/tmp");
    let genes = generate_genes(rarity).unwrap();
    let compositor = Composite::new(tmp, data);

    compositor.composite(genes);
}
