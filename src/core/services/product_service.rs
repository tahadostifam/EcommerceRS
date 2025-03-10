use std::sync::{Arc, Mutex};

use crate::core::ports::product_repository::ProductRepository;


#[derive(Clone)]
pub struct ProductService {
    pub(crate) product_repo: Arc<Mutex<dyn ProductRepository>>,
}

pub fn new_product_service(
    product_repo: Arc<Mutex<dyn ProductRepository>>,
) -> ProductService {
    ProductService {
        product_repo
    }
}

impl ProductService {
    pub fn create() {
        
    }
}