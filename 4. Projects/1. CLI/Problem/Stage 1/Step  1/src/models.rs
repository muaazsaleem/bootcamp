use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Status {
  Open,
  InProgress,
  Resolved,
  Closed,
}

#[derive(Debug, PartialEq)]
pub struct Epic {
  pub name: String,
  pub description: String,
  pub status: Status,
  pub stories: Vec<u32>,
}


impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
       Epic {
         name,
         description,
         status: Status::Open,
         stories: Vec::new(),
       }
    }
}

#[derive(Debug, PartialEq)]
pub struct Story {
  pub name: String,
  pub description: String,
  pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Story {
          name,
          description,
          status: Status::Open,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

impl Default for DBState {
  fn default() -> Self {
      Self {
        last_item_id: 0,
        epics: HashMap::new(),
        stories: HashMap::new(),
      }
  }
}


#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_epic_creation() {
   let epic = Epic::new("New Epic".to_string(), "Epic description".to_string());
   
   let expected_epic = Epic {
      name: "New Epic".to_string(),
      description: "Epic description".to_string(),
      status: Status::Open,
      stories: vec![]
   };
   
   assert_eq!(epic, expected_epic);
 }

 #[test]
 fn test_story_creation() {
    let story = Story::new("New Story".to_string(), "Story description".to_string());

    let expected_story = Story {
      name: "New Story".to_string(),
      description: "Story description".to_string(),
      status: Status::Open,
    };

    assert_eq!(story, expected_story)
 }

 #[test]
 fn test_dbstate_default_properties() {
    let db_state = DBState::default();

    assert_eq!(db_state.last_item_id, 0);
    assert!(db_state.epics.is_empty());
    assert!(db_state.stories.is_empty());
 }

 #[test]
 fn test_add_epic_to_db() {
   let mut db_state = DBState::default();
   let epic = Epic::new("New Epic".to_string(), "Epic Description".to_string());
   let epic_id = 1;
   
   db_state.epics.insert(epic_id, epic);
   db_state.last_item_id = epic_id;

   let retrieved_epic = db_state.epics.get(&epic_id).unwrap();
   
   assert_eq!(retrieved_epic.name, "New Epic".to_string());
   assert_eq!(db_state.last_item_id, 1);
   assert_eq!(db_state.stories.len(), 0);
   
 }

}
