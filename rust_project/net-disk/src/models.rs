use diesel::prelude::*;

use super::schema::user_info;
use super::schema::user_info::dsl::user_info as user_info_dsl;

use super::schema::user_request;
use super::schema::user_request::dsl::user_request as user_request_dsl;

#[derive(Queryable, Debug)]
pub struct UserRequest {
    pub id: i32,
    pub username: String,
    pub request: String,
    pub time: String,
    pub token: String,
}

#[derive(Insertable)]
#[table_name = "user_request"]
pub struct NewUserRequest {
    pub username: String,
    pub request: String,
    pub time: String,
    pub token: String,
}

impl UserRequest {
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        user_request_dsl
            .load::<UserRequest>(conn)
            .expect("Error load user request")
    }
    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        user_request_dsl
            .find(id)
            .get_result::<UserRequest>(conn)
            .ok()
    }
    pub fn by_name(name: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::user_request::dsl::username;

        user_request_dsl
            .filter(username.eq(name))
            .first::<UserRequest>(conn)
            .ok()
    }
    pub fn by_request(request_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::user_request::dsl::request;

        user_request_dsl
            .filter(request.eq(request_str))
            .first::<UserRequest>(conn)
            .ok()
    }
    pub fn by_time(time_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::user_request::dsl::time;

        user_request_dsl
            .filter(time.eq(time_str))
            .first::<UserRequest>(conn)
            .ok()
    }

    pub fn by_token(token_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::user_request::dsl::token;

        user_request_dsl
            .filter(token.eq(token_str))
            .first::<UserRequest>(conn)
            .ok()
    }

    pub fn delete_by_name(username_str: &str, conn: &SqliteConnection) -> usize {
        use super::schema::user_request::dsl::username;

        diesel::delete(user_request_dsl.filter(username.eq(username_str)))
            .execute(conn)
            .expect("Error deleting users")
    }

    pub fn create(
        username: &str,
        request: &str,
        time: &str,
        token: &str,
        conn: &SqliteConnection,
    ) -> Self {
        if UserRequest::by_name(username, &conn).is_none() {
            let new_user_request = Self::new_user_struct(username, request, time, token);
            diesel::insert_into(user_request_dsl)
                .values(&new_user_request)
                .execute(conn)
                .expect("Error saving new user");
        }

        UserRequest::by_name(username, &conn).unwrap()
    }

    fn new_user_struct(username: &str, request: &str, time: &str, token: &str) -> NewUserRequest {
        NewUserRequest {
            username: username.into(),
            request: request.into(),
            time: time.into(),
            token: token.into(),
        }
    }
}

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
    /// 得到数据库中所有的用户信息
    pub fn list(conn: &SqliteConnection) -> Vec<Self> {
        user_info_dsl
            .load::<UserInfo>(conn)
            .expect("Error load users")
    }

    // 通过id 查询用户信息
    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        user_info_dsl.find(id).get_result::<UserInfo>(conn).ok()
    }

    // 通过 username 查询用户信息
    pub fn by_username(username_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::user_info::dsl::username;

        user_info_dsl
            .filter(username.eq(username_str))
            .first::<UserInfo>(conn)
            .ok()
    }

    pub fn get_salt_by_name(username_str: &str, conn: &SqliteConnection) -> String {
        Self::by_username(username_str, conn).unwrap().salt.clone()
    }

    pub fn get_cryptpassword(username_str: &str, conn: &SqliteConnection) -> String {
        Self::by_username(username_str, conn).unwrap().cryptpassword.clone()
    }

    // 删除用户信息通过username
    pub fn delete_by_name(username_str: &str, conn: &SqliteConnection) -> usize {
        use super::schema::user_info::dsl::username;

        diesel::delete(user_info_dsl.filter(username.eq(username_str)))
            .execute(conn)
            .expect("Error deleting users")
    }

    pub fn create(
        username: &str,
        password: &str,
        salt: &str,
        cryptpassword: &str,
        conn: &SqliteConnection,
    ) -> Self {
        // 不存在时，再插入
        if Self::by_username(username, &conn).is_none() {
            let new_user = Self::new_user_struct(username, password, salt, cryptpassword);
            diesel::insert_into(user_info_dsl)
                .values(&new_user)
                .execute(conn)
                .expect("Error saving new user");
        }

        // 存在直接返回数据库中的UserInfo
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
