#![allow(unused)]

use crate::prelude::{BoundedInt, LowerBoundInt};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Meta {
    pub(crate) total: i32,
    pub(crate) limit: BoundedInt<1, 20>,
    pub(crate) page: LowerBoundInt<1>,
}
