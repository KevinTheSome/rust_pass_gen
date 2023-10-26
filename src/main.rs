use rand::{thread_rng, Rng};
use std::fs::File; //oh no

const UPPERCASE: [&str; 26] = [ "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z" ];
const LOWERCASE: [&str; 26] = [ "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z" ];
const NUMBERS: [&str; 10] = [ "0","1","2","3","4","5","6","7","8","9" ];
const SPECCHER: [&str; 29] = [ "~","`","!","@","#","$","%","^","&","*","(","","_","-","+","=","{","[","}","]","|",":","; ","<",",",">",".","?","/" ];

fn randome(mut max: i32) -> i32  {
    let mut rng = thread_rng();
    max = rng.gen_range(0..=max);
    return max;
}

fn normalize(input_string: String) -> String {
    return String::from(input_string.to_lowercase().trim());
}

fn spec_char_gen() -> String {
    match randome(3){
        0=>return String::from(UPPERCASE[randome(26) as usize]), 
        1=>return String::from(LOWERCASE[randome(26) as usize]), 
        2=>return String::from(NUMBERS[randome(9) as usize]),
        3=>return String::from(SPECCHER[randome(29) as usize]), 
        _=>return String::from("error from spec_char_gen"),

    }
}

fn non_spec_char_gen() -> String {
    match randome(2){
        0=>return String::from(UPPERCASE[randome(26) as usize]), 
        1=>return String::from(LOWERCASE[randome(26) as usize]), 
        2=>return String::from(NUMBERS[randome(9) as usize]),
        _=>return String::from("error from non_spec_char_gen"),
    }
}

fn main() {
    let mut how_many: i32 = 0;
    let mut how_long: i32 = 0;
    let mut spec_cherecter_bool: bool = false;
    let mut external_bool: bool = false;
    let mut spec_cherecter: String = String::from(" ");
    let mut external: String = String::from(" ");

    println!("\n Save it in a file or generate in cosole? (Yes for console):");
    let _ = std::io::stdin().read_line(&mut external);

    if normalize(external.clone()) == String::from("yes") || normalize(external.clone()) == String::from("y"){
        external_bool = true;
        
    } else if normalize(external.clone()) == String::from("no") ||  normalize(external.clone()) == String::from("n"){
        external_bool = false;
    }

    println!("\n Do you want to Special cherecters in the password?(yes/no):");
    let _ = std::io::stdin().read_line(&mut spec_cherecter);

    if normalize(spec_cherecter.clone()) == String::from("yes") ||normalize(spec_cherecter.clone()) == String::from("y"){
        spec_cherecter_bool = true;
    } else if normalize(spec_cherecter.clone()) == String::from("no") ||normalize(spec_cherecter.clone()) == String::from("n") {
        spec_cherecter_bool = false;
    }

    // println!("\n how many passwords?(int):");
    // let _ = std::io::stdin().read_line(&mut how_many);

    // println!("\n how long each passwords?(int):");
    // let _ = std::io::stdin().read_line(&mut how_long);  

    
    //main gen stuff
    if external_bool == true{
        if spec_cherecter_bool == true{
            spec_char_gen();
        }else{
            non_spec_char_gen();
        }

    }else{
        if spec_cherecter_bool == true{
            while how_many != 0   {
                while how_long != 0   {
                    spec_char_gen();
                    how_long = how_long - 1;
                }
                how_many = how_many - 1;
            }

        }else{
            while how_many != 0   {
                while how_long != 0   {
                    non_spec_char_gen();
                    how_long = how_long - 1;
                }
                how_many = how_many - 1;
            }

        }
    }

}
