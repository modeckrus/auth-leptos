use query::{impl_id, impl_table};

use crate::c::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

impl_id!(UserId, "user");
impl_table!(User, "user");
impl User {
    pub fn new(
        id: impl Into<UserId>,
        display_name: impl Into<String>,
        avatar_url: impl Into<Option<String>>,
    ) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            avatar_url: avatar_url.into(),
        }
    }

    pub fn admin() -> Self {
        Self::new(
            UserId::new("1"),
            "Admin".to_string(),
            "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcT_gaxAkYYDw8UfNleSC2Viswv3xSmOa4bIAQ&s".to_string(),
        )
    }
}
