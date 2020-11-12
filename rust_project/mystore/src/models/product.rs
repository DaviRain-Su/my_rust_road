use crate::schema::products;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f32,
    pub price: Option<i32>, // For a value that can be null, int Rust is an Option type that will be None when the db value is null
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f32>,
    pub price: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list() -> Self {
        // These four statements can be placed in the top, or here, your can call.
        use crate::db_connection::establish_connection;
        use crate::schema::products::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let connection = establish_connection();

        let result = products
            .limit(10)
            .load::<Product>(&connection)
            .expect("Error loading products");

        //We return a value by leaving it without a comma
        ProductList(result)
    }
}
