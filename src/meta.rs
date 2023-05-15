use crate::prelude::{BoundedInt, LowerBoundInt};

#[derive(Debug, serde::Deserialize)]
pub struct Meta {
    pub(crate) total: i32,
    pub(crate) limit: BoundedInt<1, 20>,
    pub(crate) page: LowerBoundInt<1>,
}
