use crate::core::models::category::{Category, CategoryError};

pub trait CategoryRepository: Send + Sync {
    fn create_category(
        &mut self,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> Result<Category, CategoryError>;
    fn find_category_by_id(&mut self, id: i64) -> Result<Category, CategoryError>;
    fn find_all_categories(&mut self) -> Result<Vec<Category>, CategoryError>;
    fn update_category(
        &mut self,
        category_id: i64,
        new_name: String,
        new_description: String,
        new_parent_id: Option<i64>,
    ) -> Result<Category, CategoryError>;
    fn delete_category(&mut self, category_id: i64) -> Result<(), CategoryError>;
}
