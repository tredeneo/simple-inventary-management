#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbUser {
    pub name: String,
    pub login: String,
    pub email: String,
    pub id: i32,
    pub department: String,
    pub role: String,
    pub document: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbComputer {
    pub serialnumber: String,
    pub brand: String,
    pub actual_user: String,
    pub model: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbBrand {
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbDepartment {
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbRole {
    pub name: String,
}
#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbCPU {
    pub name: String,
    pub brand: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbGPU {
    pub name: String,
    pub brand: String,
}
