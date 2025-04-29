use crate::naturals::Naturals;
use crate::naturals::Naturals::Small;
use std::ops::MulAssign;

impl MulAssign<&Naturals> for Naturals {
    fn mul_assign(&mut self, rhs: &Naturals) {
        match (&self, rhs) {
            (Small(lhs), Small(rhs)) => {
                let prod = Naturals::new(*lhs as u128 * *rhs as u128);
                *self = prod.trim();
            }
            (_, _) => {}
        }
    }
}

impl Naturals {
    pub fn pow(self, exp: &Self) -> Self {
        todo!()
    }
}
