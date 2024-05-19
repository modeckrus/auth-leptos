

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "night")]
    Night,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Night
    }
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::Night => "night",
        }
    }

    pub fn from_str(s: &str) -> Option<Self>{
        match s {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            "night" => Some(Theme::Night),
            _ => None
        }
    }

    pub fn vec() -> Vec<Theme>{
        vec![Theme::Light, Theme::Dark, Theme::Night]
    }

    pub fn list() -> Vec<String>{
        Self::vec().into_iter().map(|theme| theme.as_str().to_string()).collect()
    }
}