use restson::RestPath;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryRelationship {
    pub id: CategoryId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryParentRelationship {
    pub data: Option<CategoryRelationship>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryChildrenRelationships {
    pub data: Vec<CategoryRelationship>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryRelationships {
    pub parent: CategoryParentRelationship,
    pub children: CategoryChildrenRelationships,
}

#[derive(Serialize, Debug, Clone)]
pub struct CategoryId {
    id: String,
}

impl CategoryId {
    pub fn new(id: &str) -> CategoryId {
        CategoryId { id: id.to_string() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<'de> Deserialize<'de> for CategoryId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(CategoryId { id: s })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryData {
    pub id: CategoryId,
    pub attributes: CategoryAttributes,
    pub relationships: CategoryRelationships,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoriesListResponse {
    pub data: Vec<CategoryData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResponse {
    pub data: CategoryData,
}

impl RestPath<()> for CategoriesListResponse {
    fn get_path(_: ()) -> Result<String, restson::Error> {
        Ok(String::from("categories"))
    }
}

impl RestPath<CategoryId> for CategoryResponse {
    fn get_path(id: CategoryId) -> Result<String, restson::Error> {
        Ok(String::from("categories/") + &id.id)
    }
}
