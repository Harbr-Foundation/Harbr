use url::Url;


pub struct User {
    pub username: String,
    pub nickname: Option<String>,
    pub id: uuid::Uuid,
    pub avatar_url: Url,
    pub email: Option<String>
}