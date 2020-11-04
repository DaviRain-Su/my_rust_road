use diesel::prelude::*;


use super::schema::user_info;
use super::schema::user_info::dsl::user_info as user_info_dsl;

#[derive(Queryable, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub cryptpassword: String,
}

#[derive(Insertable)]
#[table_name = "user_info"]
pub struct NewUserInfo {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub cryptpassword: String,
}

impl UserInfo {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        user_info_dsl
            .load::<UserInfo>(conn)
            .expect("Error load users")
    }

    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        user_info_dsl.find(id).get_result::<UserInfo>(conn).ok()
    }

    pub fn by_username(username_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::user_info::dsl::username;

        user_info_dsl
            .filter(username.eq(username_str))
            .first::<UserInfo>(conn)
            .ok()
    }

    pub fn create(
        username: &str,
        password: &str,
        salt: &str,
        cryptpassword: &str,
        conn: &SqliteConnection,
    ) -> Self {

        let new_user = Self::new_user_struct(username, password, salt, cryptpassword);

        diesel::insert_into(user_info_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");

        Self::by_username(username, &conn).unwrap()
    }

    fn new_user_struct(
        username: &str,
        password: &str,
        salt: &str,
        cryptpassword: &str,
    ) -> NewUserInfo {
        NewUserInfo {
            username: username.into(),
            password: password.into(),
            salt: salt.into(),
            cryptpassword: cryptpassword.into(),
        }
    }
}
