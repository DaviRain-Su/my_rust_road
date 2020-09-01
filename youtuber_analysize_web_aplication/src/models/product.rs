use crate::schema::products;    

#[derive(Queryable,Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>,
}

#[derive(Insertable,Serialize, Deserialize)]
#[table_name="products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);


impl ProductList{
    pub fn list() -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::products::dsl::*;
        use crate::db_connection::extablish_conection;

        let connection = extablish_conection();

        let result = 
            products
                .limit(10)
                .load::<Product>(&connection)
                .expect("Error loading products");
        
        Self(result)
    }
}