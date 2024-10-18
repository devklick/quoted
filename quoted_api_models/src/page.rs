use serde::Serialize;
use serde::{self, Deserialize};

///
/// Defines the response from fetching a page of data.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct PagedResponse<T>
where
    T: Serialize,
{
    ///
    /// The page number that was fetched.
    ///
    pub page: u64,

    ///
    /// The maximum number of items on the page.
    /// Note that this may be greater than the actual number of items on the page.
    pub limit: u64,

    ///
    /// Whether or not more pages are available.
    /// E.g. if there are 11 items and page 1 is has a `limit` of 10, `has_more`` will be true.
    ///
    pub has_more: bool,

    ///
    /// The data on the page.
    ///
    pub data: Vec<T>,
}

///
/// Defines a request to fetch a page of data.
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct PagedRequest<T>
where
    T: Serialize,
{
    ///
    /// The page number to be fetched.
    /// Defaults to `1`.
    ///
    pub page: u64,

    ///
    /// The maximum number of items to include on the page.
    /// Defaults to `10``.
    ///
    pub limit: u64,

    ///
    /// The request parameters specific to the item being searched.
    ///
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

impl<T> PagedResponse<T>
where
    T: Serialize,
{
    pub fn new(page: u64, limit: u64, data: Vec<T>, has_more: bool) -> Self {
        PagedResponse::<T> {
            page,
            limit,
            data,
            has_more,
        }
    }
}
