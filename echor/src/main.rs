use clap::App;

fn main() {
    
    let matches = App::new("echor")
        .version("0.1.0")
        .author("KonstT")
        .about("rust echo")
        .get_matches();

    println!("{:#?}", matches);
}
