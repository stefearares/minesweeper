use std::process::exit;
use colored::*;
use std::char;
use std::fs::File;
use std::io::Read;
use std::env;


pub fn annotate(minefield: &[&str]) -> Vec<String> {
    
    let mut completed: Vec<String> = Vec::new();
    
    let width = minefield.len();
    let lenght= get_len_formatted(&minefield[0]); 

    if width == 0 && lenght == 0{
        completed.push("".to_string());
        return completed;
    }

    let mut helper: String = String::new();

    for i in 0..width{
        for j in 0..lenght{

            if minefield[i].chars().nth(j).unwrap() == '*'{
                helper.push('*');
            }

            if minefield[i].chars().nth(j).unwrap() == '·' || minefield[i].chars().nth(j).unwrap() == ' ' {
                let mut counter=0;

                // Top and Bottom
                if i != 0 {
                    if  minefield[i-1].chars().nth(j).unwrap() == '*'{
                        counter+=1;
                    }
                }
                if i != width-1{
                    if  minefield[i+1].chars().nth(j).unwrap() == '*'{
                        counter+=1;
                    }
                }   
                
                // Sides
                if j != 0{
                    if  minefield[i].chars().nth(j-1).unwrap() == '*'{
                        counter+=1;
                    }
                }
                if j != lenght-1{
                    if  minefield[i].chars().nth(j+1).unwrap() == '*'{
                        counter+=1;
                    }
                }

                // Diagonals
                if i != 0 && j != 0{
                    if minefield[i-1].chars().nth(j-1).unwrap() == '*'{
                        counter+=1;
                    }
                }
                if i != width-1 && j != lenght-1{
                    if minefield[i+1].chars().nth(j+1).unwrap() == '*'{
                        counter+=1;
                    }
                }

                if i != 0 && j != lenght-1{
                    if minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                        counter += 1;
                    }
                }
                if i != width-1 && j != 0 {
                    if minefield[i+1].chars().nth(j-1).unwrap() == '*' {
                        counter += 1;
                    }
                }

                // Substitutes the Character.
                if counter != 0 {
                    let counter_char=(counter + b'0') as char;
                    helper.push(counter_char);
                } 
                else if minefield[i].chars().nth(j).unwrap() == ' ' && counter == 0{
                    helper.push('·');
                } else if minefield[i].chars().nth(j).unwrap() == '·'  && counter == 0{
                    helper.push('·');
                }
            }
        }

        completed.push(helper.clone());
        helper.clear();
    }

    completed
}


pub fn get_len_formatted(format: &&str) -> usize {
    let mut count = 0;
    for c in format.chars() {
        if c != '\n' && c != '\r' {
            count += 1;
        }
    }
    count
}

pub fn input_validation(initial: &str) -> Vec<&str>{

    let split = initial.split('\n');
    let mut formatted:Vec<&str>=Vec::new();

    for i in split{
        formatted.push(i);
    }

    for format in &formatted{
        for i in format.chars()
        {
            if i != '·' && i != '*' && i != ' ' && i != '\r' {
                eprintln!("[{}]: [{}] is not valid <'.' for free space and '*' for bombs>", "Error".red().bold(), i);
                exit(1);
            }
        }
    }

    let deafult_size = get_len_formatted(&formatted[0]);

   for format in &formatted{

        let size = get_len_formatted(format);        

        if deafult_size != size {
            eprintln!("{}: the minesweeper can't have different sized rows.", "Error".red().bold());
            exit(1);
        }
    }

    formatted

}


fn vec_print(vector: Vec<String>){
    for string in vector {
        println!("{}", string);
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    // Check if a filename is provided as a command-line argument
    if args.len() < 2 {
        eprintln!("{}: {}cargo run <filename>","Usage".red().bold(),"$".green());
        exit(1);
    }

    let filename = &args[1];
    
    let initial: String = match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_err() {
                eprintln!("{}: reading file {}", "Error".red().bold(),filename);
                exit(1);
            }
            contents
        },
        Err(_) => {
            eprintln!("{}: reading file {}","Error:".red().bold() ,filename);
            exit(1);
        }
    };
    
    // With Text file in command line.
    let initial_s = initial.as_str();
    let minefield = input_validation(initial_s);  
    let result = annotate(&minefield);
    

    // With predifined string.
    /*let initial: &str =" *  * \n  *   \n    * \n   * *\n *  * \n      ";
    let minefield = input_validation(&initial);
    let result = annotate(&minefield);
    */

    vec_print(result);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speical_case_test1(){
        let initial: &str ="";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),[""])
    }

    #[test]
    fn speical_case_test2(){
        let initial: &str =" ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["·"])
    }

    #[test]
    fn speical_case_test3(){
        let initial: &str ="*";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["*"])
    }
    #[test]
    fn  cases_test1(){
        let initial: &str ="·*·*·\n··*··\n··*··\n·····";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["1*3*1","13*31","·2*2·","·111·"])

    }

    #[test]
    fn edge_cases_test2(){
        let initial: &str ="***\n***\n***\n***";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["***","***","***","***"])

    }

    #[test]
    fn edge_cases_test3(){
        let initial: &str ="   \n   \n   \n   ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["···","···","···","···"])

    }

    #[test]
    fn edge_cases_test4(){
        let initial: &str ="   \n * \n   ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["111","1*1","111"])

    }

    #[test]
    fn edge_cases_test5(){
        let initial: &str ="***\n* *\n***";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["***","*8*","***"])

    }

    #[test]
    fn edge_cases_test6(){
        let initial: &str =" * * ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["1*2*1"])

    }

    #[test]
    fn edge_cases_test7(){
        let initial: &str ="*   *";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["*1·1*"])

    }

    #[test]
    fn edge_cases_test8(){
        let initial: &str =" * * ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["1*2*1"])

    }

    #[test]
    fn edge_cases_test9(){
        let initial: &str ="*   *";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["*1·1*"])

    }

    #[test]
    fn edge_cases_test10(){
        let initial: &str ="  *  \n  *  \n*****\n  *  \n  *  ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["·2*2·","25*52","*****","25*52","·2*2·"])

    }

    #[test]
    fn edge_cases_test11(){
        let initial: &str =" *  * \n  *   \n    * \n   * *\n *  * \n      ";
        let minefield=input_validation(initial);
        assert_eq!(annotate(&minefield),["1*22*1","12*322","·123*2","112*4*","1*22*2","111111"])

    }
}