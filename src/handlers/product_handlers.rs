
use crate::models::product_models::{EditProduct, NewProduct, Product};
use crate::models::user_models::UserId;


use axum::extract::Path;

use axum::response::{IntoResponse, Json};


use image::Luma;
use reqwest::StatusCode;
use serde_json::json;
use sqlx::{query, query_as, PgPool};

use std::io::{Cursor, Seek, SeekFrom};
use std::sync::Arc;

use qrcode::QrCode;





pub async fn create_product(
    Path(user_id): Path<i32>,
    Json(new_product): Json<NewProduct>, pool: Arc<PgPool>) -> impl IntoResponse {

        let merchant_id: Vec<UserId> = query_as!(
            UserId,
            "
            SELECT id
            FROM merchants
            WHERE user_id = $1
            ",
            user_id
        )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch merchant id");

        if let Some(first_merchant_id) = merchant_id.get(0) {
            let result = query!(
                "
                INSERT INTO products_and_services (merchant_id, title, description, price, is_product, is_discounted, discounted_amount, on_sale, on_sale_amount) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                RETURNING *
                ",
                first_merchant_id.id,
                &new_product.title.as_ref().map(|s| s.as_str()).unwrap_or(""),
                &new_product.description.as_ref().map(|s| s.as_str()).unwrap_or(""),
                new_product.price, 
                new_product.is_product,
                new_product.is_discounted, 
                new_product.discounted_amount, 
                new_product.on_sale, 
                new_product.on_sale_amount
        
        
        
            )
                    .fetch_one(&*pool)
                    .await;
        
            match result {
                Ok(row) => {
                    let new_id: i32 = row.id;
                    // after successfully created, 
                    // use the product data to generate a qrcode image
                    //save this qrcode image and product_id to the qr_codes table
                    // Generate QR code image
                

                // let qr_code_data = Product {
                //     id: row.id,
                //     merchant_id: row.merchant_id,
                //     title: row.title,
                //     description: row.description,
                //     price: Some(row.price),
                //     is_product: row.is_product,
                //     is_discounted: row.is_discounted,
                //     discounted_amount: row.discounted_amount,
                //     on_sale: row.on_sale,
                //     on_sale_amount: row.on_sale_amount,
                //     created_at: row.created_at,
                //     edited_at: row.edited_at
 
                // };
                let qr_code_data = format!(
                    "https://example.com/product/{}?id={}&merchant_id={}&price={}&is_product={}&is_discounted={}&on_sale={}",
                    new_id,
                    row.id,
                    row.merchant_id,
                    // row.title,
                    // row.description,
                    row.price,
                    row.is_product,
                    row.is_discounted,
                    // row.discounted_amount,
                    row.on_sale,
                    // row.on_sale_amount
                );
            
                let qr_code_data_json = serde_json::to_string(&qr_code_data).unwrap();
                let qr_code = QrCode::new(qr_code_data_json.as_bytes()).unwrap();
                let qr_image = qr_code.render::<Luma<u8>>().build();

                // Convert the QR code image to a base64-encoded string
                let qr_image_data = base64::encode(&qr_image.clone().into_raw());


                // qr_code.render::<Luma<u8>>()
    // .build()
    // .write_to(&mut *qr_image, image::ImageFormat::WebP)
    // .unwrap();

    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    
    
// Clone qr_image before passing it to write_to
qr_image
    .clone()
    .write_to(&mut buffer, image::ImageFormat::WebP)
    .unwrap();


// Seek back to the beginning of the buffer
buffer.seek(SeekFrom::Start(0)).unwrap();
// Create a cursor from the buffer to read its content
let cursor = Cursor::new(buffer.clone());


    axum::response::Response::builder().header("content-type", "image/webp").body(buffer.into_inner());


                    // return Json(json!({
                    //     "status": "success",
                    //     "message": "Product added successfully",
                    //     "new_id": new_id, 
                    //     "qr_image": qr_image_data
                    // }));
                }
                Err(e) => {
                    return Json(json!({
                        "status": "error",
                        "message": format!("Failed to create product : {:?}", e)
                    }));
                }
            }
        }
        Json(json!([]))

}


pub async fn get_merchant_products(
    Path(merchant_id): Path<i32>,
    pool: Arc<PgPool>) -> impl IntoResponse {

    
    
    let product: Vec<Product> = query_as!(
        Product,
        "
        SELECT * FROM products_and_services WHERE merchant_id = $1
        ",
        merchant_id

    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch product");

    

    Json(product)
}


pub async fn update_product(
    Path(product_id): Path<i32>,
    Json(product_data): Json<EditProduct>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    // Update the product details in the database
    let result = sqlx::query(
        "UPDATE products_and_services SET title = $1, description = $2, price = $3, is_discounted = $4, discounted_amount = $5, is_product = $6, on_sale = $7, on_sale_amount = $8 WHERE id = $9",
    )
    .bind(&product_data.title)
        .bind(&product_data.description)
        .bind(product_data.price)
        .bind(product_data.is_discounted)
        .bind(product_data.discounted_amount)
        .bind(product_data.is_discounted)
        .bind(product_data.on_sale)
        .bind(product_data.on_sale_amount)
        .bind(product_id)
        .execute(&*pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json("Product details updated")).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to update product details: {}", e)),
        )
            .into_response(),
    }
}


pub async fn delete_product(Path(product_id): Path<i32>, pool: Arc<PgPool>) -> impl IntoResponse {
    let result = query!(
        "
            DELETE FROM products_and_services
            WHERE id = $1
            RETURNING id
            ",
        product_id
    )
    .fetch_one(&*pool)
    .await;

    match result {
        Ok(row) => {
            let deleted_id = row.id;
            return Json(json!({
                "status": "success",
                "message": "Product deleted successfully",
                "deleted_id": deleted_id
            }));
        }
        Err(e) => {
            println!("Error deleting product: {:?}", e);
            return Json(json!({
                "status": "error",
                "message": format!("Failed to delete product: {:?}", e)
            }));
        }
    }
}



//QRCode

pub async fn get_qr_code(
    Path(product_id): Path<i32>,
    // Json(payload): Json<CreateQrCodePayload>,
    pool: Arc<PgPool>,
) -> impl IntoResponse {
    // Get the product details from the database
    let product_result:Vec<Product>= query_as!(
        Product,
        "SELECT *
         FROM products_and_services p
         WHERE p.id = $1",
         product_id
    )
    .fetch_all(&*pool)
    .await
    .expect("Failed to fetch product id");

    if let Some(first_product) = product_result.get(0) {

        let qr_code_data = format!(
            "https://example.com/product/{}?id={}&merchant_id={}&is_product={}&is_discounted={}&on_sale={}",
            first_product.id,
            first_product.id,
            first_product.merchant_id,
            // first_product.title,
            // first_product.description,
            // first_product.price,
            first_product.is_product,
            first_product.is_discounted,
            // first_product.discounted_amount,
            first_product.on_sale,
            // first_product.on_sale_amount
        );
    
        // Create the QR code
        let qr_code = QrCode::new(qr_code_data.as_bytes()).unwrap();
        let qr_image = qr_code.render::<Luma<u8>>().build();


    //     let mut reader = BufReader::new(qr_image);
    // let mut buffer = Vec::new();

    // // Read file into vector.
    // reader.read_to_end(&mut buffer).unwrap();
    // ([("content-type", "image/png; charset=utf-8")], buffer).into_response()
    // let stream = ReaderStream::new(qr_image);

        

        

        // Return the JSON response with the QR code image
        // 
        
        // Ok(IntoResponse::Ok().json(response_json))


        let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    
    
        // Clone qr_image before passing it to write_to
        qr_image
            .clone()
            .write_to(&mut buffer, image::ImageFormat::WebP)
            .unwrap();
        
        
        // Seek back to the beginning of the buffer
        buffer.seek(SeekFrom::Start(0)).unwrap();
        // Create a cursor from the buffer to read its content
        let cursor = Cursor::new(buffer.clone());


        print!("qr code generated");
        
        
            axum::response::Response::builder().header("content-type", "image/webp").body(buffer.into_inner());


            
        

      

    

    }


    print!("cococococo");

    Json(json!([]))

   
}