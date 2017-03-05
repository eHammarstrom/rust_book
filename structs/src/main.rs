struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/* lifetime @Ch10
struct LifetimeUser {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
*/

fn main() {
    let user = User {
        email: String::from("an.email@example.com"),
        username: String::from("an.email"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user.email);

    /* lifetime @Ch10
    let lifetimeUser = LifetimeUser {
        email: "an.email@example.com",
        username: "an.email",
        active: true,
        sign_in_count: 1,
    };

    println!("{}", lifetimeUser.email);
    */
}
