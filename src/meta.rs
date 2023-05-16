use crate::prelude::{BoundedInt, LowerBoundInt};

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Meta {
    pub(crate) _total: i32,
    pub(crate) _limit: BoundedInt<1, 20>,
    pub(crate) _page: LowerBoundInt<1>,
}
