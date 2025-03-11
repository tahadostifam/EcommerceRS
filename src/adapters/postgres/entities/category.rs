use crate::{adapters::postgres::schema::*, core::models::category::Category};
use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::*};

#[derive(Debug, Selectable, Queryable, Insertable)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(Pg))]
pub struct CategoryEntity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub parent_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CategoryEntity {
    pub fn to_model(&self) -> Category {
        Category {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            parent_id: self.parent_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(Pg))]
pub struct NewCategoryEntity {
    pub name: String,
    pub description: String,
    pub parent_id: Option<i64>,
}
