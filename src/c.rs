pub type ID = String;

pub type IDRef<'a> = &'a str;

pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub fn now() -> Timestamp {
    chrono::Utc::now()
}

pub fn make_id() -> ID {
    nanoid::nanoid!()
}

pub async fn sleep(duration: std::time::Duration) {
    #[cfg(feature = "ssr")]
    tokio::time::sleep(duration).await;
    #[cfg(feature = "hydrate")]
    async_std::task::sleep(duration).await;
}
