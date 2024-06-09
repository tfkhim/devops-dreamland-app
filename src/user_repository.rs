use std::collections::HashMap;

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

    pub async fn get_display_name_for_username(&self, username: &str) -> Option<String> {
        self.users
            .get(username)
            .map(|user_data| user_data.display_name.clone())
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
