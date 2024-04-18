use std::process::exit;
use std::env as console;
use colored::*;
use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    
    let mut completed: Vec<String> = Vec::new();
    
    let width = minefield.len() ;
    let lenght= minefield[0].chars().count();
    
    let mut helper: String=String::new();

    let l_width = width -1;
    let l_lenght = lenght -1;

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
                
                if i != l_width{
                    if  minefield[i+1].chars().nth(j).unwrap() == '*'{
                        counter+=1;
                    }}   
                
                if j != 0{
                    if  minefield[i].chars().nth(j-1).unwrap() == '*'{
                        counter+=1;
                    }
                }

                if j != l_lenght{
                    if  minefield[i].chars().nth(j+1).unwrap() == '*'{
                        counter+=1;
                    }
                }

                if i != 0 && j != 0{
                    if minefield[i-1].chars().nth(j-1).unwrap() == '*'{
                        counter+=1;
                    }
                }

                if i != l_width && j != l_lenght{
                    if minefield[i+1].chars().nth(j+1).unwrap() == '*'{
                        counter+=1;
                    }
                }

                if i != 0 && j != l_lenght && minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                    counter += 1;
                }
                if i != l_width && j != 0 && minefield[i+1].chars().nth(j-1).unwrap() == '*' {
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

    for i in initial.chars()
    {
        if i != '·' && i != '*' && i != '\n' && i!=' '{
            eprintln!("{}: {} is not valid <'·',' ' for free space and '*' for bombs>","Error".red().bold(),i);
            exit(1);
        }
    }


    let split=initial.split('\n');
    let mut formatted:Vec<&str>=Vec::new();

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
    if initial.is_empty() || initial.len()==1{
        println!("{:}",initial);
    }
    else{
    let new= initial.replace(' ', "·");
    let minefield=input_validation(&new);
    let completed=annotate(&minefield);
    
    for i in completed{
        println!("{}",i);
    }}
}