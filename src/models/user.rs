use axum_login::{secrecy::SecretVec, AuthUser};

#[derive(Debug, Clone)]
pub struct User {
    id: usize,
    name: String,
    password_hash: String,
    role: Role,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Role {
    User,
    Admin,
}

impl AuthUser<usize, Role> for User {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }

    fn get_role(&self) -> Option<Role> {
        Some(self.role.clone())
    }
}
