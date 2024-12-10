use alloc::sync::Arc;
use alloc::{string::String, vec, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref AUTHENTICATOR: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![User::new(
        String::from("fantasypvp"),
        String::from("password")
    )]));
};

pub struct User {
    username: String,
    pass_hash: u64,
}

impl User {
    fn new(username: String, password: String) -> User {
        let pass_hash = User::get_pass_hash(&password);
        User {
            username,
            pass_hash,
        }
    }

    fn get_pass_hash(pass: &String) -> u64 {
        pass.bytes().fold(0u64, |b, a| a as u64 + b as u64)
    }
    pub fn is_authenticated(username: String, password: String) -> Option<bool> {
        let auth = AUTHENTICATOR.lock();
        let user = auth.iter().find(|x| x.username == username)?;
        if User::get_pass_hash(&password) == user.pass_hash {
            Some(true)
        } else {
            Some(false)
        }
    }
}
