use std::env;
//use configparser::ini::Ini;




fn main() {

    /* Find Current Directory */
    //let local_dir = env::current_dir().unwrap(); Its a buffer. Lets change to string
    let local_dir: String = String::from(env::current_dir().unwrap().to_string_lossy());
    let config_file_name = "config.ini".to_string();


    let result = [local_dir, config_file_name].join("/");

    println!("{}", result);

    /* Can recreate these variables but what is the best practice ?  */
    //let local_dir: String = String::from(env::current_dir().unwrap().to_string_lossy());
    //let config_file_name = "config.ini".to_string();
    println!("debug:{:?}",local_dir);
    println!("debug:{:?}",config_file_name);
    
}
