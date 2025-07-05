// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount {
    name: String,
    age: Option<u32>,
}

// define a trait called Balance, and within, function get_balance returning integer of 10
trait Balance {
    fn get_balance(&self) -> u32; 
}

impl Balance for UserAccount {
    fn get_balance(&self) -> u32 {
        10
    }
}

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
fn increase_balance<T: Balance>(user: &T, amount: u32) -> Result<u32,String> {
    // within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
    if amount <= 10 {
        Ok(user.get_balance() + amount)
        // - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
        // Tip: this function should return a Result<u32, String>
    } else {
        Err("Increase must be less than 10!".to_owned())
    }

}


fn main() {
    // create user_account, and set his age as Option::None
    let user_account = UserAccount {
        name: "Tony Stark".to_owned(),
        age: None,
    };
    // You want to increase the user_account's balance by 11 // Tried changing amount to a number less than 10 to see result
    let result = increase_balance(&user_account, 8);

    match result {
        // Ok: print "UserAccount balance increased to {}" where {} is the new balance value
        Ok(new_balance) => println!("User balance increased to {}", new_balance),
        // Err: print the error message returned
        Err(msg) => println!("{}", msg),
    };
    
    if let Some(age) = user_account.age {
        println!("UserAccount age is {}", age)
    } 

}
