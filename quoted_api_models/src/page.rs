use serde::Serialize;
use serde::{self, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PagedData<T>
where
    T: Serialize,
{
    pub page: u64,
    pub limit: u64,
    pub has_more: bool,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagedRequest<T>
where
    T: Serialize,
{
    pub page: u64,
    pub limit: u64,
    #[serde(flatten)]
    pub query: T,
}

impl<T> Default for PagedRequest<T>
where
    T: Serialize,
    T: Default,
{
    fn default() -> Self {
        Self {
            page: 1,
            limit: 10,
            query: Default::default(),
        }
    }
}

impl<T> PagedData<T>
where
    T: Serialize,
{
    pub fn new(page: u64, limit: u64, data: Vec<T>) -> Self {
        PagedData::<T> {
            page,
            limit,
            data,
            has_more: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetShowsRequest {}

impl Default for GetShowsRequest {
    fn default() -> Self {
        Self {}
    }
}
