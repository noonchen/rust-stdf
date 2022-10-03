mod stdf_file;
mod stdf_error;
mod stdf_types;

fn main() {
    println!("Hello, world!");
    let f = match stdf_file::StdfReader::new("") {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;}
    };
    println!("{:?}", f);
}

