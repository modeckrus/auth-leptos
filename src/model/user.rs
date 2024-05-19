use crate::c::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: ID,
    pub display_name: String,
    pub login: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    pub fn new(
        id: impl Into<ID>,
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
            make_id(),
            login.clone(),
            login,
            password,
            "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg".to_string(),
            now(),
            now(),
        )
    }

    pub fn admin() -> Self {
        Self::new(
            "1".to_string(),
            "Admin".to_string(),
            "a@a.a".to_string(),
            "a".to_string(),
            "https://daisyui.com/images/stock/photo-1534528741775-53994a69daeb.jpg".to_string(),
            now(),
            now(),
        )
    }
}
