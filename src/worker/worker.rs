use rayon::ThreadPoolBuilder;

use crate::{
    compositor::generate::Composite,
    genes::generate::{generate_parts, genes_view},
};

pub fn worker() {
    let threads = num_cpus::get();
    let pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    let total_imgs: usize = 10_000;
    let parts = (35, 20, 15, 12, 8, 6, 4);
    let genes_map = generate_parts(parts, total_imgs);

    pool.scope(|scope| {
        let keys = genes_map.keys();

        for key in keys {
            let list = genes_map.get(&key).unwrap();

            for (index, gene) in list.iter().enumerate() {
                scope.spawn(move |_| {
                    let out = format!("/home/rinat/source/tmp/{}", &key);
                    let data = String::from("/home/rinat/source/dragons_imgs");
                    let compositer = Composite::new(out, data);

                    println!("{}", genes_view(&gene));
                    compositer.composite(&gene, index);
                });
            }
        }
    });
}
