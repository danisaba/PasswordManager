use std::io;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use nanoid::nanoid;

//This function sets up the box. It gives you your options and takes you to the functions which will do the options for you.
fn text_options(hash: &mut HashMap<String, String>){
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
                save_password(hash);
            }else if input.eq("make a random password\n") {
                make_random_password(hash)
            }else if input.eq("modify a password\n") {
                modify_existing_password(hash)
            }else if input.eq("quit\n"){
                println!("Cya later!");
                std::process::exit(0);
            }else{
                println!("That is not an option!");
            }
        },
        Err(e) => println!("Hm, we have an error: {}", e)
    }
}

//this function generates a 16 character password
fn make_16_char_pass(special_chars: bool) -> String {
    let alphabet: [char; 82] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','!','@','#','$','%','^','&','*','(',')','_','-','+','=',';',':','/','?','<','>'
    ];
    if special_chars {
        let pass = nanoid!(16, &alphabet);
        return pass
    }else{
        let pass: String = thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
        return pass
    }
}

//this function generates a 32 character password
fn make_32_char_pass(special_chars: bool) -> String{
    let alphabet: [char; 82] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','!','@','#','$','%','^','&','*','(',')','_','-','+','=',';',':','/','?','<','>'
    ];
    if special_chars {
        let pass = nanoid!(32, &alphabet);
        return pass
    }else{
        let pass: String = thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();
        return pass
    }
}

/*
this function is going to make new passwords. It will ask if you want a random 32 or 16 character pass
and then it will ask if you want special characters. Once all your options are set it will give you a random pass
and then it will ask if you want to save it or if you want to generate a new one. if you want to save it it will
ask what app this pass is for and then add it to the Hashmap of app/pass.
*/
fn make_random_password(hash: &mut HashMap<String, String>){
    println!("Would you like a 16 or 32 character password? (16/32)");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.eq("quit\n") {
                println!("Ok. Bye bye :-)");
                std::process::exit(0);
            }
            println!("Next, would you like special characters(/,!,@, etc) in this password? (y/n)");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input.eq("16\ny\n"){
                        println!("Sweet! Let's make a 16 char password with special characters!");
                        let password = make_16_char_pass(true);
                        println!("Here is your generated password: {}", password);
                        generate_cont(hash, password);
                    }else if input.eq("16\nn\n"){
                        println!("Okay! Let's make a 16 char password without special characters!");
                        let password = make_16_char_pass(false);
                        println!("Here is your generated password: {}", password);
                        generate_cont(hash, password);
                    }else if input.eq("32\ny\n"){
                        println!("Alright. Let's make a 32 char password with special characters!");
                        let password = make_32_char_pass(true);
                        println!("Here is your generated password: {}", password);
                        generate_cont(hash, password);
                    }else if input.eq("32\nn\n"){
                        println!("Okay! Let's make a 32 char passord without special characters.");
                        let password = make_32_char_pass(false);
                        println!("Here is your generated password: {}", password);
                        generate_cont(hash, password);
                    }else if input.eq("16\nquit\n") || input.eq("32\nquit\n"){
                        println!("Aw man! You were in the middle of generating. Bye!");
                        std::process::exit(0);
                    }else {
                        println!("That is not an option!");
                    }
                },
                Err(e) => println!("That doesn't seem to be an option!: {}", e)
            }
        },
    Err(e) => println!("Hm, that doesn't seem to be an option we have!: {}", e)
    }
}

//this function serves to make generating less long since its a big one.
fn generate_cont(hash: &mut HashMap<String, String>, password: String) {
    println!("Would you like to save this password or generate a new one? Enter 'save' to save, and 'exit' to leave the generator!");
    let mut input2 = String::new();
    match io::stdin().read_line(&mut input2) {
        Ok(_) => {
            if input2.trim().eq("save"){
                println!("Okay, let's save this password :-). What application/website are you saving this password for?");
                let mut input3 = String::new();
                match io::stdin().read_line(&mut input3) {
                    Ok(_) => {
                        hash.insert(input3.trim().to_string(), password);
                        println!("Okie dokie! Your new password has been stored :-)");
                    },
                    Err(e) => println!("That doesn't seem to be an option!: {}", e)
                }
            }else if input2.trim().eq("exit") {
                return
            }
        },
        Err(e) => println!("That doesn't seem to be an option!: {}", e)
    }
}


//If a user has an already thought-up password to save, they can save it with this function
fn save_password(hash: &mut HashMap<String, String>) {
    println!("What is the application/website/etc you are saving the password for?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input_text = input.trim();
            if input_text.eq("quit"){
                println!("Oh. Bye!");
                std::process::exit(0);
            }
            println!("Perfect! What do you want the password for {} to be", input_text);
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input_list: Vec<&str>;
                    input_list = input.split("\n").collect();
                    if input_list[1].eq("quit"){
                        println!("Dang. Right in the middle of something!");
                        std::process::exit(0);
                    }
                    println!("Okay, so {}'s password is {}. Saved!", input_list[0], input_list[1]);
                    hash.insert(input_list[0].to_string(), input_list[1].to_string());
                },  
                Err(e) => println!("Hm. Looks like that might be incorrect: {}", e)
            }
        },
        Err(e) => println!("Hm. Looks like that might be incorrect: {}", e)
    }
}

/*
This function will go through the hashmap and be able to modify a value inside of it, so that the modified
password can be saved instead of making a new key for it.
*/
fn modify_existing_password(hash: &mut HashMap<String, String>){
    println!("What is the name of the application/website that you are trying to modify the saved password for?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().eq("quit") {
                println!("Okay! Bye bye :-)");
                std::process::exit(0);
            }
            match hash.get(input.trim()) {
                Some(password) => println!("Your current saved password is {}.", password),
                _ => {
                    println!("Hm. Looks like that isn't a current entry!");
                    std::process::exit(0);
                }
            }
            println!("What would you like to change the password to?");
            let mut input2 = String::new();
            match io::stdin().read_line(&mut input2) {
                Ok(_) => {
                    if input2.trim().eq("quit"){
                        println!("Right in the middle of something...");
                        std::process::exit(0);
                    }
                    println!("Perfect! We will change the password to {}", input2.trim());
                    hash.remove(input.trim());
                    hash.insert(input.trim().to_string(), input2.trim().to_string());
                },
                Err(e) => println!("Hm. Looks like that might be incorrect: {}", e)
            }
        },
        Err(e) => println!("Hm. Looks like that might be incorrect: {}", e)
    }
}

// This will print all of the currently saved passwords from the dictionary in "application: password" format
fn print_saved_passwords(hash: &mut HashMap<String, String>){
    println!("Here are your current saved passwords: ");
    for (app, pass) in hash {
        println!("{}: \"{}\"", app, pass);
    }
}

//This is the main function. It will put everything together
fn main() {
    let mut passwords_saved: HashMap<String, String> = HashMap::new();
    println!("Welcome to Password Box! Soon to have usernames and passwords! Enter quit at any time to leave our box. Enter anything to continue!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            while !input.trim().eq("quit") {
            text_options(&mut passwords_saved);
            }
        },
        Err(e) => println!("Hm. Looks like that might be incorrect: {}", e)
    }
}
