use dotenv::dotenv;

pub fn check_dotenv() {
    dotenv().ok();

    println!("Dotenv successfully compiled.");
    return ()
}

#[cfg(test)]
mod tests {
    use dotenv;
    use std::env;
    
    #[test]
    fn test_check_dotenv() {
        dotenv::from_path("/../.env").ok();
        let envs = ["MONGO_URL"];
        println!("{}", env::var("MONGO_URL").unwrap());

        envs.iter().map(|envKey| env::var(envKey).unwrap());

        return ()
    }
}