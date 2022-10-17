pub mod HelpPrint {
    use dotenv;
    use std::env;
    use std::path::Path;
    // print object type
    pub fn print_type_of<T>(_: &T) {
        println!("tipo de dato: {}", std::any::type_name::<T>())
    }

    pub fn get_nas_folder() -> String {
        let dirname = env::var("CARGO_MANIFEST_DIR").unwrap();
        let nas_env_value = dotenv::var("NAS_ROOT_FOLDER_NAME").unwrap();
        
        let nas_path = Path::new(&dirname).join(nas_env_value).display().to_string();
        nas_path
    }
}
