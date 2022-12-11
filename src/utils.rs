pub fn get_path_seperator() -> &'static str {
    let mut path_seperator = "";
    #[cfg(target_os = "windows")]
    {
        path_seperator = "\\";
    }
    #[cfg(target_os = "macos")]
    {
        path_seperator = "/";
    }
    #[cfg(target_os = "linux")]
    {
        path_seperator = "/";
    }

    if path_seperator == "" {
        panic!("unknown os");
    }

    path_seperator
}

pub fn build_input_path(day: u32, filename: &str) -> String {
    let mut path = String::from("dec");
    path.push_str(day.to_string().as_str());
    path.push_str(get_path_seperator());
    path.push_str(filename);
    path
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut path = String::new();
    #[cfg(target_os = "windows")]
    {
        path.push_str("C:\\Users\\phil-\\Documents\\code\\rust\\adventofcode\\src\\");
    }
    #[cfg(target_os = "linux")]
    {
        path.push_str("/mnt/c/Users/phil-/Documents/code/rust/adventofcode/src/");
    }
    path.push_str(filename);
    let filename = path.as_str();

    let result = std::fs::read_to_string(filename).unwrap().lines().map(|s| s.to_string()).collect::<Vec<String>>();
    result
}