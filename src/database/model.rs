#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbUser {
    pub name: String,
    pub login: String,
    pub email: String,
    pub id: i32,
}
#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbComputer {
    pub serialnumber: String,
    pub brand: String,
    pub actual_user: String,
    pub model: String,
}
