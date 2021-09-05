use std::io;
use std::collections::HashMap;

//This function sets up the box. It gives you your options and takes you to the functions which will do the options for you.
fn text_options(hash: &mut HashMap<&str, &str>){
    println!("Enter 'see passwords' if you would like your current saved passwords listed!");
    println!("Enter 'save a password' if you would like to save a new password to your vault!");
    println!("Enter 'make a random password' if you would like a generated password!");
    println!("Enter 'modify a password' if you would like to change a currently saved password!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.eq("see passwords\n") {
                print_saved_passwords(hash)
            }else if input.eq("save a password\n"){
                 save_password()
            }else if input.eq("make a random password\n") {
                make_new_password()
            }else if input.eq("modify a password\n") {
                modify_existing_password()
            }else{
                println!("That is not an option!");
                std::process::exit(0);
            }
        },
        Err(e) => println!("Hm, we have an error: {}", e)
    }
}

/*
this function is going to make new passwords. It will ask if you want a random 32 or 16 character pass
and then it will ask if you want special characters. Once all your options are set it will give you a random pass
and then it will ask if you want to save it or if you want to generate a new one. if you want to save it it will
ask what app this pass is for and then add it to the Hashmap of app/pass.
*/
fn make_new_password(){
    println!("Would you like a 16 or 32 character password? (16/32)");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Next, would you like special characters(/,!,@, etc) in this password? (y/n)");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    println!("{}", input);
                    if input.eq("16\ny\n"){
                        println!("Sweet! Let's make a 16 char password with special characters!");
                    }else if input.eq("16\nn\n"){
                        println!("Okay! Let's make a 16 char password without special characters!");
                    }else if input.eq("32\ny\n"){
                        println!("Alright. Let's make a 32 char password with special characters!");
                    }else if input.eq("32\nn\n"){
                        println!("Okay! Let's make a 32 char passord without special characters.");
                    }else{
                        println!("That is not an option!");
                        std::process::exit(0);
                    }
                    println!("Here is your generated password: ");
                    //add password print
                    println!("Would you like to save this password or generate a new one? Enter 'save' to save, and nothing to generate another!")
                    //if else
                },
                Err(e) => println!("That doesn't seem to be an option!: {}", e)
            }
        },
    Err(e) => println!("Hm, that doesn't seem to be an option we have!: {}", e)
    }
}


//If a user has an already thought-up password to save, they can save it with this function
fn save_password(){
    println!("we savin");
/*
    println!("What is the application you are saving the password for?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {

        },
        Err(e) => println!("Hm. Looks like that might be incorrect: {}", e)
    }
    //make this the key
    println!("What is the password you are saving?");
    //make this the value
    println!("Password saved!");
*/
}

/*
This function will go through the hashmap and be able to modify a value inside of it, so that the modified
password can be saved instead of making a new key for it.
*/
fn modify_existing_password(){
    println!("Which password would you like to modify?");
    //if else. If they enter something applicable, move forward, if not, print that it is not a current hash key
    //cannot implement until the hashmap is done.
}

// This will print all of the currently saved passwords from the dictionary in "application: password" format
fn print_saved_passwords(hash: &mut HashMap<&str, &str>){
    println!("Here are your current saved passwords: ");
    for (app, pass) in hash {
        println!("{}: \"{}\"", app, pass);
    }
}

//This is the main function. It will put everything together
fn main() {
    let mut passwords_saved: HashMap<&str, &str> = HashMap::new();
    println!("Welcome to Password Box! Soon to have usernames and passwords! Please enter one of the following options!");
    text_options(&mut passwords_saved);
}
