use std::io;

fn main() {

    loop {
        println!("If you want to convert Fahrenheit to Celsius, type 1");
        println!("If you want to convert Celsius to Fahrenheit, type 2");
        println!("If you want to quit, type 3");

        println!("Please type your choice : ");

        let mut user_input = String::new();

        io::stdin() 
            .read_line(&mut user_input)      
            .expect("Failed to read line");
       
        let user_input : u32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {

                println!("Please choose between 1 and 2, or quit by typing 3");
                continue;
            }
        };

        if user_input == 1 {
            println!("Type the Fahrenheit degree you want to convert : ");
            
            let mut f_input = String::new();
            
            io::stdin() 
            .read_line(&mut f_input)      
            .expect("Failed to read line");
       
            let f_input : u32 = match f_input.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            let f_to_c : u32 = (f_input - 32) * 5/9;
            println!("{f_input} in Celsius is {f_to_c}");
            continue;

        } else if user_input == 2 {
            
            println!("Type the Celsius degree you want to convert :");
            
            let mut c_input = String::new();
            
            io::stdin() 
            .read_line(&mut c_input)      
            .expect("Failed to read line");
       
            let c_input : u32 = match c_input.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            let c_to_f : u32 = (c_input * 9/5) + 32;
            println!("{c_input} in Fahrenheit is {c_to_f}");
            continue;

        } else if user_input == 3 {
            break;
        }
        
        else {
            println!("Please choose between option 1 and option 2 ");
            continue;
        }
    }
}