//This function's only purpose is to make Main not as clunky.
fn text_options(){
    println!("Enter 'see passwords' if you would like your current saved passwords listed!");
    println!("Enter 'save password' if you would like to save a new password to your vault!");
    println!("Enter 'random password' if you would like a generated password!");
    println!("Enter 'modify password' if you would like to change a currently saved password!");
}

/*
this function is going to make new passwords. It will ask if you want a random 32 or 16 character pass
and then it will ask if you want special characters. Once all your options are set it will give you a random pass
and then it will ask if you want to save it or if you want to generate a new one. if you want to save it it will
ask what app this pass is for and then add it to the Hashmap of app/pass.
*/
fn make_new_password(){
    println!("Would you like a 16 or 32 character password?");
    //if else
    println!("Would you like special characters(/,!@ etc) in the password?");
    //if else
    println!("Here is your generated password: ");
    //add password print
    println!("Would you like to save this password or generate a new one? Enter 'save' to save, and nothing to generate another!")
    //if else
}


//If a user has an already thought-up password to save, they can save it with this function
fn save_password(){
    println!("What is the application you are saving the password for?");
    //make this the key
    println!("What is the password you are saving?");
    //make this the value
    println!("Password saved!");
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
fn print_saved_passwords(){
    println!("Here are your current saved passwords: ");
    //printing current hashmap cannot be done until I have implemented the hashmap
}

//This is the main function. It will put everything together
fn main() {
    println!("Welcome to Password Box! Soon to have usernames and passwords! Please enter one of the following options!");
    text_options();
    
}