use std::sync::{Arc, Mutex};

use crate::core::{
    models::category::{Category, CategoryError},
    ports::category_repository::CategoryRepository,
};
#[derive(Clone)]
pub struct CategoryService {
    pub(crate) category_repo: Arc<Mutex<dyn CategoryRepository>>,
}

pub fn new_category_service(category_repo: Arc<Mutex<dyn CategoryRepository>>) -> CategoryService {
    CategoryService { category_repo }
}

impl CategoryService {
    pub fn create(
        &mut self,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> Result<Category, CategoryError> {
        let mut category_repo = self.category_repo.lock().unwrap();
        let category = category_repo.create_category(name, description, parent_id)?;
        Ok(category)
    }

    pub fn get_all(&mut self) -> Result<Vec<Category>, CategoryError> {
        let mut category_repo = self.category_repo.lock().unwrap();
        // let categories: Vec<Category> = category_repo.;
        Ok(vec![])
    }

    pub fn get(&mut self, category_id: i64) -> Result<Category, CategoryError> {
        let mut category_repo = self.category_repo.lock().unwrap();
        let category = category_repo.find_category_by_id(category_id)?;
        Ok(category)
    }

    pub fn update(
        &mut self,
        category_id: i64,
        new_name: String,
        new_description: String,
        new_parent_id: Option<i64>,
    ) -> Result<Category, CategoryError> {
        let mut category_repo = self.category_repo.lock().unwrap();
        let category =
            category_repo.update_category(category_id, new_name, new_description, new_parent_id)?;
        Ok(category)
    }

    pub fn delete(&mut self, category_id: i64) -> Result<(), CategoryError> {
        let mut category_repo = self.category_repo.lock().unwrap();
        category_repo.delete_category(category_id)?;
        Ok(())
    }
}
