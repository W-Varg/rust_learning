// #[macro_use] extern crate base64;
extern crate base64;

pub mod MsFileService {
    use crate::{utils::HelpPrint::get_nas_folder, InputData};

    use base64::{decode, encode};

    use rocket::serde::json::Json;
    use std::{
        fs::File,
        io::{Read, Write},
        path::Path,
    };

    pub fn saveBase64(mut input: Json<InputData>) {
        if input.withDateSubFolder == true {
            let date = chrono::Utc::now().format("%Y/%m/%d").to_string();
            let path = Path::new(&input.path)
                .join(date)
                .to_string_lossy()
                .replace("//", "/")
                .to_string();
            input.path = path;
        }
        println!("{:?}", &input.path);
        let nas_location = &get_nas_folder();
        let path = Path::new(nas_location).join(&input.path);

        println!("nas dir {:?}", &path);

        // const destination = libPath.join(getNasFolder(), toPath);
        // let mut my_file = File::create(path.join("uploads/file.pdf")).unwrap();

        let mut my_file = File::create("uploads/file.pdf").unwrap();
        let bytes = &decode(&input.base64).unwrap();
        my_file.write_all(bytes).unwrap();
    }
}
