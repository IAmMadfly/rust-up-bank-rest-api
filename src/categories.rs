use serde::{Deserialize, Serialize};
use restson::{RestPath};



#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryAttributes {
    name:           String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryRelationship {
    id:             CategoryId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryParentRelationship {
    data:           Option<CategoryRelationship>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryChildrenRelationships  {
    data:           Vec<CategoryRelationship>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryRelationships {
    parent:         CategoryParentRelationship,
    children:       CategoryChildrenRelationships
}

#[derive(Serialize, Debug)]
pub struct CategoryId {
    id:             String
}

impl CategoryId {
    pub fn new(id: &str) -> CategoryId {
        CategoryId {
            id:     id.to_string()
        }
    }
}

impl<'de> Deserialize<'de> for CategoryId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(CategoryId {
            id:     s
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryData {
    id:             CategoryId,
    attributes:     CategoryAttributes,
    relationships:  CategoryRelationships
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoriesListResponse {
    data:   Vec<CategoryData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResponse {
    data:   CategoryData
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

