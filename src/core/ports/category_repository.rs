use crate::core::models::category::{Category, CategoryError};

pub trait CategoryRepository : Send + Sync{
    fn create_category(&mut self, category: Category) -> Result<Category, CategoryError>;
    fn find_category_by_id(&mut self, id: i64) -> Result<Category, CategoryError>;
    fn find_all_categories(&mut self) -> Result<Vec<Category>, CategoryError>;
    fn update_category(&mut self, category: Category) -> Result<Category, CategoryError>;
    fn delete_category(&mut self, id: i64) -> Result<(), CategoryError>;
    fn find_categories_by_name(&mut self, name: &str) -> Result<Vec<Category>, CategoryError>;
}