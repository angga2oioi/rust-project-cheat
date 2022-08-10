use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id:Option<u32>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct UserQueryParams {
    search: Option<String>,
    sort_by: Option<String>,
    limit: Option<i32>,
    page: Option<i32>,
}