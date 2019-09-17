use serde::{Deserialize, Serialize};
use std::error::Error;

/// User objects allow you to manage a user and the user’s attributes such as a department, phone number, employee number, email, and username. The API allows you to create, delete, retrieve a user or a list of users, and update user information
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct User {
    /// The domo user id
    pub id: Option<u32>,

    /// User's full name
    pub name: Option<String>,

    /// User's primary email used in profile
    pub email: Option<String>,

    /// User's secondary email in profile
    pub alternate_email: Option<String>,

    /// Employee number within company
    pub employee_number: Option<u32>,

    /// User's job title
    pub title: Option<String>,

    /// Primary phone number of user
    pub phone: Option<String>,

    /// Free text that can be used to define office location (e.g. City, State, Country)
    pub location: Option<String>,

    /// Time zone used to display to user the system times throughout Domo application
    pub timezone: Option<String>,

    /// Locale used to display to user the system settings throughout Domo application
    pub locale: Option<String>,

    /// The role of the user created (available roles are: 'Admin', 'Privileged', 'Participant')
    /// Deprecated in liu of custom roles and authorities
    pub role: Option<String>,

    /// If the user ID is related to a user that has been deleted, a subset of the user information will be returned, including a deleted property, which will be true.
    pub deleted: Option<bool>,
}

impl User {
    pub fn new() -> Self {
        User {
            id: None,
            name: None,
            email: None,
            alternate_email: None,
            employee_number: None,
            title: None,
            phone: None,
            location: None,
            timezone: None,
            locale: None,
            role: None,
            deleted: None,
        }
    }
    pub fn template() -> Self {
        User {
            id: Some(0),
            name: Some(String::from("First Last")),
            email: Some(String::from("First.Last@company.com")),
            alternate_email: Some(String::from("first.last@gmail.com")),
            employee_number: Some(0),
            title: Some(String::from("Title")),
            phone: Some(String::from("+1 (800) 700-6000")),
            location: Some(String::from("CA")),
            timezone: Some(String::from("America/Los_Angeles")),
            locale: Some(String::from("en-US")),
            role: Some(String::from("Admin - Match roles defined in instance")),
            deleted: Some(false),
        }
    }
}

/// User API methods
/// Uses the form method_object
impl super::Client {
    /// Get a list of users.
    pub fn get_users(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = offset {
            q.push(("offset", v.to_string()));
        }
        Ok(self
            .client
            .get(&format!("{}{}", self.host, "/v1/users"))
            .query(&q)
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Fetch users by email in bulk
    pub fn post_bulk_user_emails(&self, emails: &[String]) -> Result<Vec<User>, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .post(&format!("{}{}", self.host, "/v1/users/bulk/emails"))
            .header("Authorization", at)
            .json(emails)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Creates a new user in your Domo instance.
    ///
    /// TODO param sendInvite=true
    pub fn post_user(&self, user: User) -> Result<User, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .post(&format!("{}{}", self.host, "/v1/users"))
            .header("Authorization", at)
            .json(&user)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Retrieves the details of an existing user.
    ///
    /// Returns a user object if valid user ID was provided. When requesting, if the user ID is related to a user that has been deleted, a subset of the user information will be returned, including a deleted property, which will be true.
    pub fn get_user(&self, id: &str) -> Result<User, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .get(&format!("{}{}{}", self.host, "/v1/users/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Updates the specified user by providing values to parameters passed. Any parameter left out of the request will cause the specific user’s attribute to remain unchanged
    /// Currently all user fields are required
    pub fn put_user(&self, id: &str, user: User) -> Result<User, Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        Ok(self
            .client
            .put(&format!("{}{}{}", self.host, "/v1/users/", id))
            .header("Authorization", at)
            .json(&user)
            .send()?
            .error_for_status()?
            .json()?)
    }

    /// Permanently deletes a user from your Domo instance
    /// This is destructive and cannot be reversed.
    pub fn delete_user(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let at = self.get_access_token("user")?;
        self.client
            .delete(&format!("{}{}{}", self.host, "/v1/users/", id))
            .header("Authorization", at)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}