use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.file_name)?;

  println!("\n\n{}\n\n", content);

    
  let result = case_insensitive(&config.query, &content);


  println!("Result: ");
  for line in result {
    println!("\t{}", line);
  }
  
  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_name: String
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, &str> {
      if args.len() < 3 {
         return Err("Arguments are not enough to run the program!!");
      }

      let query = args[1].clone();
      let file_name = args[2].clone();

      Ok(Self { query, file_name })
  }
}

pub fn case_sensitive<'a>(query: &String, data: &'a String) -> Vec<&'a str> {
  let mut result: Vec<&'a str> = Vec::new();
  for line in data.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
} 

pub fn case_insensitive<'a>(query: &String, data: &'a String) -> Vec<&'a str> {
  let mut result: Vec<&'a str> = Vec::new();
  for line in data.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      result.push(line);
    }
  }
  result
} 



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive() {
      let query: String = "doc".to_string();
      let data = " hi,
Where are you??
doc?? are you there??".to_string();

      let mut result = Vec::new();
      result.push("doc?? are you there??");

        assert_eq!(result, case_sensitive(&query, &data));
    }

    #[test]
    fn insensitive() {
      let query = "doc".to_string();
      let data = " hi,
Where are you??
doc?? are you there??
Doctor!!!".to_string();

      let mut result = Vec::new();
      result.push("doc?? are you there??");
      result.push("Doctor!!!");

        assert_eq!(result, case_insensitive(&query, &data));
    }
}