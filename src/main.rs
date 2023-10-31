use rand::{thread_rng, Rng};
use std::fs::File; //oh no
use std::fs; 

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

fn write_to_file(input_string:String) -> std::io::Result<()> {
    File::create("password.txt")?;
    fs::write("password.txt",input_string)?;
    Ok(())
}

fn main() {
    let mut how_many_string: String = String::from(" ");
    let mut how_long_string: String = String::from(" ");
    let mut spec_cherecter_bool: bool = false;
    let mut external_bool: bool = false;
    let mut spec_cherecter: String = String::from(" ");
    let mut external: String = String::from(" ");

    println!("\n Save it in a file or generate in cosole? (No for console):");
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

    println!("\n how many passwords?(int):");
    let _ = std::io::stdin().read_line(&mut how_many_string);
    let how_many: i32 = how_many_string.trim().parse().unwrap(); //form string to int

    println!("\n how long each passwords?(int):");
    let _ = std::io::stdin().read_line(&mut how_long_string);
    let how_long: i32 = how_long_string.trim().parse().unwrap(); //form string to int

    //chad :https://stackoverflow.com/questions/27043268/convert-a-string-to-int/54733404#54733404

    //main gen stuff
    let mut buffer:String = String::new();
    let new_line:String = String::from("\n");

    let mut how_long_clone:i32;
    let mut how_many_clone:i32;

    if external_bool == true{
        if spec_cherecter_bool == true{
            how_many_clone = how_many;
            while how_many_clone != 0 || how_many_clone < 0   {
                how_long_clone = how_long;
                while how_long_clone != 0 || how_long_clone < 0   {
                    let mut temp:String = String::from(spec_char_gen());
                    buffer = format!("{buffer}{temp}");
                    how_long_clone = how_long_clone - 1;
                }
                buffer = format!("{buffer}{new_line}");
                how_long_clone = how_long;
                how_many_clone = how_many_clone - 1;
            }
        }else{
            how_many_clone = how_many;
            while how_many_clone != 0 || how_many_clone < 0   {
                how_long_clone = how_long;
                while how_long_clone != 0 || how_long_clone < 0   {
                    let mut temp:String = String::from(non_spec_char_gen());
                    buffer = format!("{buffer}{temp}");
                    how_long_clone = how_long_clone - 1;
                }
                buffer = format!("{buffer}{new_line}");
                how_long_clone = how_long;
                how_many_clone = how_many_clone - 1;
            }
        }
        let _ = write_to_file(buffer);
    }else{
        if spec_cherecter_bool == true{
            how_many_clone = how_many;
            while how_many_clone != 0 || how_many_clone < 0   {
                how_long_clone = how_long;
                while how_long_clone != 0 || how_long_clone < 0   {
                    let mut temp:String = String::from(spec_char_gen());
                    buffer = format!("{buffer}{temp}");
                    how_long_clone = how_long_clone - 1;
                }
                println!("{}",buffer);
                buffer = String::new();
                how_long_clone = how_long;
                how_many_clone = how_many_clone - 1;
            }
        }else{
            how_many_clone = how_many;
            while how_many_clone != 0 || how_many_clone < 0   {
                how_long_clone = how_long;
                while how_long_clone != 0 || how_long_clone < 0   {
                    let mut temp:String = String::from(non_spec_char_gen());
                    buffer = format!("{buffer}{temp}");
                    how_long_clone = how_long_clone - 1;
                }
                println!("{}",buffer);
                buffer = String::new();
                how_long_clone = how_long;
                how_many_clone = how_many_clone - 1;
            }
        }
    }

}
