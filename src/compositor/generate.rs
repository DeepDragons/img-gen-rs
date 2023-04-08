use image::{imageops, DynamicImage, GenericImageView, ImageError, RgbaImage};
use std::path::Path;

use crate::{config::general::DEFAULT_FORMAT, genes};

pub struct Composite {
    out: String,
    data: String,
}

impl Composite {
    pub fn new(out: String, data: String) -> Self {
        Composite { out, data }
    }

    pub fn composite(self, genes: Vec<u8>) {}

    fn open_background(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[1];
        let mask = format!("{}/background/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_aura(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[12];
        let mask = format!("{}/aura/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_body(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[2];
        let mask = format!("{}/body/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_eyes(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[3];
        let mask = format!("{}/eyes/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_horns(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[4];
        let mask = format!("{}/horns/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_mouth(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[5];
        let mask = format!("{}/mouth/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_spine(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[6];
        let mask = format!("{}/spine/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_chest(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[7];
        let mask = format!("{}/chest/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_wings(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[8];
        let mask = format!("{}/wings/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_accessories(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[9];
        let mask = format!("{}/accessories/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_ears(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[10];
        let mask = format!("{}/ears/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }

    fn open_hair(self, genes: Vec<u8>) -> Result<DynamicImage, ImageError> {
        let gen = genes[11];
        let mask = format!("{}/hair/{}.{}", self.data, gen, DEFAULT_FORMAT);

        image::open(&mask)
    }
}
