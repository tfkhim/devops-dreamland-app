use std::{collections::HashMap, io::ErrorKind};

pub struct UserRepository {
    users: HashMap<String, UserData>,
}

struct UserData {
    id: String,
    display_name: String,
}

impl UserRepository {
    pub fn new() -> Self {
        let users = vec![UserData::new("alice", "Alice"), UserData::new("bob", "Bob")];

        UserRepository {
            users: users
                .into_iter()
                .map(|user| (user.id.clone(), user))
                .collect(),
        }
    }

    pub async fn get_display_name_by_id(
        &self,
        user_id: &str,
    ) -> Result<Option<String>, std::io::Error> {
        if user_id == "error" {
            return Err(ErrorKind::ConnectionReset.into());
        }

        let display_name = self
            .users
            .get(user_id)
            .map(|user_data| user_data.display_name.clone());

        Ok(display_name)
    }
}

impl UserData {
    fn new(id: &str, display_name: &str) -> Self {
        UserData {
            id: id.to_owned(),
            display_name: display_name.to_owned(),
        }
    }
}
