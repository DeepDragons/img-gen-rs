use img_gen_rs::{
    compositor::generate::Composite,
    genes::generate::{generate_genes, genes_view},
};

fn main() {
    let rarity = 6;
    let data = String::from("/home/rinat/source/dragons_imgs");
    let tmp = String::from("/home/rinat/source/tmp");
    let genes = generate_genes(rarity).unwrap();
    let compositor = Composite::new(tmp, data);
    let genes_str = genes_view(&genes);

    println!("{:?}", genes_str);

    compositor.composite(genes);
}
