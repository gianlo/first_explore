use super::super::utils::ints;
use super::super::typeclasses::{Colorizable, Color};

#[derive(Debug)]
pub struct NationalGDP {
    pub in_local_currency: u32
}

impl Colorizable for NationalGDP {
    fn color(&self) -> Color {
        let value = ints::linearscale(self.in_local_currency);
        Color {
            red : value,
            blue : value,
            green : value,
            alpha: 255}
    }
}
