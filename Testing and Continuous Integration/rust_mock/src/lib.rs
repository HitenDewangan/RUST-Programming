// Trait defining the dependency (e.g., a data repository)
trait UserRepository {
    fn get_user_by_id(&self, id: u32) -> Option<String>;
    fn add_user(&mut self, name: &str) -> Result<(), String>; // Added a mutable method
}

// Real implementation of the UserRepository (using a database, for example)
struct DatabaseUserRepository;

impl UserRepository for DatabaseUserRepository {
    fn get_user_by_id(&self, id: u32) -> Option<String> {
        // Code to fetch user from the database
        println!("Fetching user {} from database...", id); // Simulate database interaction
        Some("User from database".to_string()) // Placeholder
    }

    fn add_user(&mut self, name: &str) -> Result<(), String> {
        // Code to add user to the database
        println!("Adding user {} to database...", name); // Simulate database interaction
        Ok(()) // Placeholder
    }
}

// Code that uses the UserRepository trait
fn greet_user(user_repository: &dyn UserRepository, user_id: u32) -> String {
    if let Some(user_name) = user_repository.get_user_by_id(user_id) {
        format!("Hello, {}!", user_name)
    } else {
        "User not found".to_string()
    }
}

fn create_and_greet_user(
    user_repository: &mut dyn UserRepository,
    name: &str,
    user_id: u32,
) -> String {
    let _ = user_repository.add_user(name);
    greet_user(user_repository, user_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation of UserRepository
    struct MockUserRepository {
        user_name: Option<String>,
        added_users: Vec<String>, // Store added users
    }

    impl MockUserRepository {
        fn new() -> Self {
            MockUserRepository {
                user_name: None,
                added_users: Vec::new(),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        fn get_user_by_id(&self, _id: u32) -> Option<String> {
            self.user_name.clone()
        }

        fn add_user(&mut self, name: &str) -> Result<(), String> {
            self.added_users.push(name.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_greet_user() {
        // Create a mock user repository
        let mock_repo = MockUserRepository {
            user_name: Some("Test User".to_string()),
            added_users: Vec::new(),
        };

        // Call greet_user with the mock
        let greeting = greet_user(&mock_repo, 1);
        assert_eq!(greeting, "Hello, Test User!");

        // Test when the user is not found
        let mock_repo = MockUserRepository {
            user_name: None,
            added_users: Vec::new(),
        };
        let greeting = greet_user(&mock_repo, 1);
        assert_eq!(greeting, "User not found");
    }

    #[test]
    fn test_create_and_greet_user() {
        let mut mock_repo = MockUserRepository::new();
        let greeting = create_and_greet_user(&mut mock_repo, "New User", 1);
        assert_eq!(greeting, "Hello, New User!");
        assert_eq!(mock_repo.added_users.len(), 1);
        assert_eq!(mock_repo.added_users[0], "New User");

        let mut mock_repo = MockUserRepository::new();
        mock_repo.user_name = None; // Simulate user not found
        let greeting = create_and_greet_user(&mut mock_repo, "Another User", 2);
        assert_eq!(greeting, "User not found");
        assert_eq!(mock_repo.added_users.len(), 1); // User should still be added
    }
}

fn main() {
    // Example usage with the real database repository
    let db_repo = DatabaseUserRepository;
    let greeting = greet_user(&db_repo, 123);
    println!("{}", greeting);

    let mut db_repo = DatabaseUserRepository;
    let greeting = create_and_greet_user(&mut db_repo, "Real User", 456);
    println!("{}", greeting);
}
