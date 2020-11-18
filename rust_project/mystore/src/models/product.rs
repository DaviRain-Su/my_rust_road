use crate::schema::products;
use crate::schema::products::dsl::products as products_dsl;
use diesel::prelude::*;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f32,
    pub price: Option<i32>, // For a value that can be null, int Rust is an Option type that will be None when the db value is null
}

impl Product {
    pub fn by_id(id: &i32, connection: &MysqlConnection) -> Result<Product, diesel::result::Error> {
        // let connection = establish_connection();
        products_dsl.find(id).first(connection)
    }
    pub fn delete(id: &i32, connection: &MysqlConnection) -> Result<(), diesel::result::Error> {
        // let connection = establish_connection();
        diesel::delete(products_dsl.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_product: &NewProduct, connection: &MysqlConnection) -> Result<(), diesel::result::Error> {
        // let connection = establish_connection();

        diesel::update(products_dsl.find(id))
            .set(new_product)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
    pub fn list(connection: &MysqlConnection) -> Self {
        // let connection = establish_connection();
        let result = products_dsl
            .limit(10)
            .load::<Product>(connection)
            .expect("Error loading products");

        //We return a value by leaving it without a comma
        ProductList(result)
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
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
    pub fn create(&self, connection: &MysqlConnection) -> Option<Product> {
        // let connection = establish_connection();

        diesel::insert_into(products_dsl)
            .values(self)
            .execute(connection)
            .expect("Error create product");

        Self::by_name(&self.name, connection)
    }

    pub fn by_name(name_str: &str, conn: &MysqlConnection) -> Option<Product> {
        use crate::schema::products::dsl::name;

        products_dsl
            .filter(name.eq(name_str))
            .first::<Product>(conn)
            .ok()
    }
}
