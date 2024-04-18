use std::process::exit;
use std::env as console;
use colored::*;
use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    
    let mut completed: Vec<String> = Vec::new();
    
    let width = minefield.len() ;
    let lenght= minefield[0].chars().count();
    
    let mut helper: String=String::new();

    for i in 0..width{
        for j in 0..lenght{

            if minefield[i].chars().nth(j).unwrap() == '*'{
                helper.push('*');
            }

            if minefield[i].chars().nth(j).unwrap() == '·' || minefield[i].chars().nth(j).unwrap() == ' ' {
                let mut counter=0;

                if i != 0 && minefield[i-1].chars().nth(j).unwrap() == '*' {
                    counter+=1;
                }
                
                if i != width-1{
                    if  minefield[i+1].chars().nth(j).unwrap() == '*'{
                        counter+=1;
                    }}   
                
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

                if i != 0 && j != lenght-1 && minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                    counter += 1;
                }
                if i != width-1 && j != 0 && minefield[i+1].chars().nth(j-1).unwrap() == '*' {
                    counter += 1;
                }

                if counter != 0 {
                    let mut counter_char=(counter + b'0') as char;
                    helper.push(counter_char);
                } else {
                    helper.push('·');
                }
            }
        }

        completed.push(helper.clone());
        helper.clear();
    }

 completed
}

pub fn input_validation(initial: &str) -> Vec<&str>{

    let mut formatted:Vec<&str>=Vec::new();

    for i in initial.chars()
    {
        if i != '·' && i != '*' && i != '\n' && i!=' '{
            eprintln!("{}: {} is not valid <'·',' ' for free space and '*' for bombs>","Error".red().bold(),i);
            exit(1);
        }
    }

    let split=initial.split('\n');
    
    for i in split{
        formatted.push(i);
    }
 
    let lenght=formatted[0].chars().count();
   
    for i in &formatted{
        let verifier=i.chars().count();

        if verifier != lenght {
            eprintln!("{}: the minesweeper can't have different sized rows.","Error".red().bold());
            exit(1);
        }
    }
     
    formatted

}


fn main() {
    
    let initial: &str ="*\n \n*\n ";

    let new= initial.replace(' ', "·");
    let minefield=input_validation(&new);
    let completed=annotate(&minefield);
    
}

#[cfg(test)]
mod tests {
    use super::*;

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