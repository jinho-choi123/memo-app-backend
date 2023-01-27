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
    use crate::utils::check_dotenv::check_dotenv;
    
    #[test]
    fn test_check_dotenv() {
        check_dotenv();
        let envs = ["MONGO_URL"];
        println!("{:?}", env::var("MONGO_URL").unwrap());

        envs.iter().map(|envKey| env::var(envKey).unwrap());

        return ()
    }
}