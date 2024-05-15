#[derive(sqlx::FromRow, Debug)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub text: String,
}