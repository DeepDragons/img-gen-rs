use image::{imageops, DynamicImage};

use crate::config::general::DEFAULT_FORMAT;

pub struct Composite {
    out: String,
    data: String,
}

impl Composite {
    pub fn new(out: String, data: String) -> Self {
        Composite { out, data }
    }

    pub fn composite(self, genes: &Vec<u8>) {
        let mut background = self
            .open_background(&genes)
            .unwrap_or(DynamicImage::new_rgba8(2500, 2500));
        let save_path = format!("{}/{}.{}", self.out, 0, DEFAULT_FORMAT);
        let some_pars = vec![
            self.open_back_details(&genes),
            self.open_wings(&genes),
            self.open_body(&genes),
            self.open_aura(&genes),
            self.open_eyes(&genes),
            self.open_horns(&genes),
            self.open_mouth(&genes),
            self.open_spine(&genes),
            self.open_chest(&genes),
            self.open_accessories(&genes),
            self.open_ears(&genes),
            self.open_hair(&genes),
        ];
        let pars: Vec<&DynamicImage> = some_pars.iter().filter_map(|v| v.as_ref()).collect();

        for img in pars {
            background = self.overlay_images(&background, &img);
        }

        background.save(save_path).unwrap();
    }

    fn overlay_images(&self, background: &DynamicImage, foreground: &DynamicImage) -> DynamicImage {
        let mut result = background.to_rgba8();

        imageops::overlay(&mut result, foreground, 0, 0);

        DynamicImage::ImageRgba8(result)
    }

    fn open_background(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[1];
        let rarity = genes[0];
        let mask = format!(
            "{}/{}/Background/{}.{}",
            self.data, rarity, gen, DEFAULT_FORMAT
        );

        image::open(mask).ok()
    }

    fn open_aura(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[12];
        let rarity = genes[0];
        let mask = format!("{}/{}/Aura/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_back_details(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[13];
        let rarity = genes[0];
        let mask = format!(
            "{}/{}/Backdetails/{}.{}",
            self.data, rarity, gen, DEFAULT_FORMAT
        );

        image::open(mask).ok()
    }

    fn open_body(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[2];
        let rarity = genes[0];
        let mask = format!("{}/{}/Body/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_eyes(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[3];
        let rarity = genes[0];
        let mask = format!("{}/{}/Eyes/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_horns(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[4];
        let rarity = genes[0];
        let mask = format!("{}/{}/Horns/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_mouth(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[5];
        let rarity = genes[0];
        let mask = format!("{}/{}/Mouth/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_spine(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[6];
        let rarity = genes[0];
        let mask = format!("{}/{}/Spine/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_chest(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[7];
        let rarity = genes[0];
        let mask = format!("{}/{}/Chest/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_wings(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[8];
        let rarity = genes[0];
        let mask = format!("{}/{}/Wings/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_accessories(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[9];
        let rarity = genes[0];
        let mask = format!(
            "{}/{}/Accessories/{}.{}",
            self.data, rarity, gen, DEFAULT_FORMAT
        );

        image::open(mask).ok()
    }

    fn open_ears(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[10];
        let rarity = genes[0];
        let mask = format!("{}/{}/Ears/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }

    fn open_hair(&self, genes: &Vec<u8>) -> Option<DynamicImage> {
        let gen = genes[11];
        let rarity = genes[0];
        let mask = format!("{}/{}/Hair/{}.{}", self.data, rarity, gen, DEFAULT_FORMAT);

        image::open(mask).ok()
    }
}
