// Entry point
// точка входа
use memethos::Config;

fn main() {
    let per = Config::load_from_file();
    println!("{:?}", per)
}
