use crate::core::models::category::{Category, CategoryError};

pub trait CategoryRepository {
    fn create_category(&self, category: Category) -> Result<Category, CategoryError>;
    fn find_category_by_id(&self, id: i64) -> Result<Category, CategoryError>;
    fn find_all_categories(&self) -> Result<Vec<Category>, CategoryError>;
    fn update_category(&self, category: Category) -> Result<Category, CategoryError>;
    fn delete_category(&self, id: i64) -> Result<(), CategoryError>;
    fn find_categories_by_name(&self, name: &str) -> Result<Vec<Category>, CategoryError>;
}