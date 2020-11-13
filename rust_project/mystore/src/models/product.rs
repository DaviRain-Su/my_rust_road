use diesel::prelude::*;
use crate::schema::products;
use crate::schema::products::dsl::products as products_dsl;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f32,
    pub price: Option<i32>, // For a value that can be null, int Rust is an Option type that will be None when the db value is null
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list() -> Self {
        // These four statements can be placed in the top, or here, your can call.
        use crate::db_connection::establish_connection;

        let connection = establish_connection();

        let result = products_dsl
            .limit(10)
            .load::<Product>(&connection)
            .expect("Error loading products");

        //We return a value by leaving it without a comma
        ProductList(result)
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub stock: f32,
    pub price: Option<i32>,
}

impl NewProduct {
    // Take a look at the method definition, I'm borrowing self.
    // just for fun remove the & after writing the handler and
    // take a look at the error, to make it work we would need to use into_inner (https://actix.rs/api/actix-web/stable/actix_web/struct.Json.html#method.into_inner)
    // which points to the inner value of the Json request.
    // pub fn create(&self) -> Result<Product, diesel::result::Error> {
    //     use crate::db_connection::establish_connection;
    //
    //     let connection = establish_connection();
    //
    //     diesel::insert_into(products_dsl)
    //         .values(self)
    //         .get_result(&connection)
    // }
}
