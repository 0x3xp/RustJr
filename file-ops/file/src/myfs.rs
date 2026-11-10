use std::fs;

pub fn test_create_dir() {
    let path = "./data";
    let del_dir = "./del";
    
    let directory_path_check = std::path::Path::new(path);
    if directory_path_check.exists() {
        println!("Directory already exists.");
        return;
    }

    let create_dir_result = fs::create_dir(path);
    let del_dir_result = fs::create_dir(del_dir);

    if create_dir_result.is_ok() {
        println!("Created new dir successfull: {}.", path);
    } else {
        println!("Problem occured: {:?}", create_dir_result.err());
    }

    if del_dir_result.is_ok() {
        println!("Created new dir successfull: {}", del_dir);
    } else {
        println!("Problem occured creating {del_dir}: {:?}", del_dir_result.err());
    }
}

pub fn create_files() {
   let path = "./data/file.txt";
   let path2 = "./data/file2.txt";
   let text = "RUST RUST RUST RUST";
   let text2 = "HELLO HELLO HELLO HELLO";
   
   _ = std::fs::write(path, text);    
   _ = std::fs::write(path2, text2);

   let file_remove_result = std::fs::remove_file(path2);
   if file_remove_result.is_ok() {
       println!("Successfully removed file: {}", path2);
   } else {
       println!("File not removed: {:?}", file_remove_result.err());
   }
}

pub fn remove_dir() {
    let path = "./del";
    let delete_del_dir = std::fs::remove_dir_all(path);

    if delete_del_dir.is_ok() {
        println!("Successfully removed {path}");
    } else {
        println!("Problem occured can not remove dir: {path}");
    }
}

pub fn read_file() {
    let file = "./data/file.txt";
    let read_result = std::fs::read(file);

    let converted_text = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        _ = a.push(new_char);
        return a;
    };

    if read_result.is_ok() {
        println!("File read: {file} includes content: {:?}", read_result.ok().unwrap().iter().fold(String::from(""), converted_text));
    }
}
