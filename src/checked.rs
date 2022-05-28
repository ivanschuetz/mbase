use anyhow::Result;

pub trait CheckedAdd: Sized {
    fn add(&self, o: &Self) -> Result<Self>;
}

pub trait CheckedSub: Sized {
    fn sub(&self, o: &Self) -> Result<Self>;
}

pub trait CheckedMul: Sized {
    fn mul(&self, o: &Self) -> Result<Self>;
}

pub trait CheckedDiv: Sized {
    fn div(&self, o: &Self) -> Result<Self>;
}

pub trait CheckedMulOther<Rhs = Self>: Sized {
    fn mul(self, rhs: Rhs) -> Result<Self>;
}
