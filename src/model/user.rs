use query::{impl_id, impl_table};

use crate::c::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub login: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl_id!(UserId, "user");
impl_table!(User, "user");
impl User {
    pub fn new(
        id: impl Into<UserId>,
        display_name: impl Into<String>,
        login: impl Into<String>,
        password: impl Into<String>,
        avatar_url: impl Into<Option<String>>,
        created_at: Timestamp,
        updated_at: Timestamp,
    ) -> Self {
        Self {
            id: id.into(),
            display_name: display_name.into(),
            login: login.into(),
            password: password.into(),
            avatar_url: avatar_url.into(),
            created_at,
            updated_at,
        }
    }

    pub fn login_pass(login: impl Into<String>, password: impl Into<String>) -> Self {
        let login = login.into();
        let password = password.into();
        Self::new(
            UserId::none(),
            login.clone(),
            login,
            password,
            None,
            now(),
            now(),
        )
    }

    pub fn admin() -> Self {
        Self::new(
            UserId::new("1"),
            "Admin".to_string(),
            "a@a.a".to_string(),
            "a".to_string(),
            "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcT_gaxAkYYDw8UfNleSC2Viswv3xSmOa4bIAQ&s".to_string(),
            now(),
            now(),
        )
    }
}
