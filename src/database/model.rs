#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbUser {
    pub name: String,
    pub login: String,
    pub email: String,
    pub id: i32,
    pub department: String,
    pub document: String,
    pub extension: String,
    pub phone_number: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbComputer {
    pub serialnumber: String,
    pub storage: i32,
    pub memory: i32,
    pub model: String,
    pub observation: String,
    pub actual_user: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbSmartphone {
    pub serialnumber: String,
    pub storage: i32,
    pub memory: i32,
    pub model: String,
    pub observation: String,
    pub actual_user: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbLastUser {
    pub usuario: String,
    pub date_begin: String,
    pub date_end: Option<String>,
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
pub struct DbEquipamentModel {
    pub name: String,
    pub brand: String,
    pub cpu: String,
    pub gpu: String,
    pub smartphone: i32,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbGPU {
    pub name: String,
    pub brand: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
pub struct DbInteger {
    pub id: i32,
}
