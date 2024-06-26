use walkdir::WalkDir;
use std::io;

pub fn input(text : &String) -> String {

    let mut input = String::new();
    println!("{}", text); 
   
    io::stdin().read_line(&mut input).unwrap(); 
    
    return input;

}

fn main() {
   
    let path = input(&String::from("Enter search path: ")); 
    let user_file_inp = input(&String::from("Enter Target name: "));

    let mut return_paths : Vec<String> = vec![];

    for entry in WalkDir::new(path.as_str().trim()).into_iter().filter_map(|e| e.ok()) {
        let entry = entry.path().display();
   
        if entry.to_string().contains(user_file_inp.as_str().trim()) {
            return_paths.push(entry.to_string());
        }
    
    }
    
    for i in return_paths.into_iter() {
        println!("{}", i);
    }
}



