use std::time::SystemTime;

#[derive(Debug)]
struct Unauthorized;

#[derive(Debug)]
struct Authorized {
    authorized_since: SystemTime,
}

#[derive(Debug)]
struct User<State> {
    name: String,
    state: State,
}

impl User<Unauthorized> {
    fn new<S: Into<String>>(name: S) -> Self {
        User {
            name: name.into(),
            state: Unauthorized,
        }
    }

    fn login(self, pwd: &str) -> Result<User<Authorized>, User<Unauthorized>> {
        Ok(User {
            name: self.name,
            state: Authorized {
                authorized_since: SystemTime::now(),
            },
        })
    }
}

impl User<Authorized> {
    fn logout(self) -> Result<User<Unauthorized>, User<Authorized>> {
        Ok(User {
            name: self.name,
            state: Unauthorized,
        })
    }
}

fn main() {
    // Only has the method `login`.
    let user = User::<Unauthorized>::new("Foo");
    // `login` consumes `user` and produces an User<Authorized>
    let auth_user = user.login("bar").expect("Unable to login");
    let unauth_user = auth_user.logout().expect("Unable to logout");
}
