mod config;
mod library;
mod parser;

fn main() {
    let home = std::env::var("HOME").expect("HOME not set");
    let config = config::parse(format!("{}/.config/attune/config.toml", home)).unwrap();
    println!("{:?}", config);
    let library = library::Library::new(config.music_library_path);
    println!("{:?}", library);
}
