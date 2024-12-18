    use std::fs;
    use std::error::Error;
    use std::env;
    use std::process;

    pub struct Config {
        pub search: String,
        pub file_path: String,
     }
     
    impl Config {
         pub fn build(
             mut args: impl Iterator<Item = String>,
         ) -> Result<Config, &'static str> {
             // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
             args.next();
     
             let search = match args.next() {
                 Some(arg) => arg,
                 None => return Err("Didn't get a query string"),
             };
     
             let file_path = match args.next() {
                 Some(arg) => arg,
                 None => return Err("Didn't get a file path"),
             };
     
             Ok( Config { search, file_path })
         }
     }
     
    pub fn run(config: Config) ->Result<(), Box<dyn Error>> {
         println!("Searching for '{}' in file: {}\n", config.search, config.file_path);
     
         let contents = fs::read_to_string(config.file_path)?;
     
         for line in search(&config.search, &contents) {
             println!("{line}");
         }
     
         Ok(())
     }
     
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
         contents.lines().filter(|line| line.contains(query)).collect()
     
         // let mut results = Vec::new();
         // for line in contents.lines() {
         //     if line.contains(query) {
         //         results.push(line);
         //     }
         // }
     
         // results
     }

    pub fn main() {
        let config = Config::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
        if let Err(err) = run(config) {
            eprintln!("Application error: {}", err);
            process::exit(1);
        }
     }
