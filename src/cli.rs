use std::{fs::File, io::{BufReader, prelude::*}};

pub fn get_args() -> Vec<String> {
    std::env::args().collect()
}

pub fn separate(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut atribs = Vec::new();
    let mut params = Vec::new();
    for arg in args {
        if arg.chars().collect::<Vec<_>>()[0] == '-' {
            atribs.push(arg);
            continue;
        }
        params.push(arg);
    }
    (atribs, params)
}

pub fn manage_atributes(atribs: Vec<String>) {
    for atrib in atribs {
        match atrib.as_str() {
            "-h" | "--help" => 
                println!("slide-ui\rcontrols:\r< >     switch slides (left and right arrow)\rq esc   quit\ratribs\r-h --help     get this message\r"),
            _ => {}
        }
    }
}

pub fn read_file(path: &str) -> Result<String, std::io::Error>{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content  = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}
