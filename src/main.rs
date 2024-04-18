use std::process::exit;
use std::env as console;
use colored::*;
use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    
    let mut completed: Vec<String> = Vec::new();
    
    let width =minefield.len();
    let lenght=minefield[0].chars().count();
    
    let mut helper: String=String::new();

    for i in 0..=width-1{
        for j in 0..=lenght-1{
          if minefield[i].chars().nth(j).unwrap() == '*'{
            helper.push('*');
          }
          if minefield[i].chars().nth(j).unwrap() == '·'{
            if i==0 && j==0 {
                let mut counter=0;
                if  minefield[i].chars().nth(j+1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i+1].chars().nth(j).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i+1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
                
            }else if  i==width-1 && j==lenght-1{
                
                let mut counter=0;

                if  minefield[i].chars().nth(j-1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i-1].chars().nth(j).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i-1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }

            }else if i==width-1 && j==0 {
                
                let mut counter=0;

                if  minefield[i].chars().nth(j+1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i-1].chars().nth(j).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
            }else if i==0 && j==lenght-1 {
                let mut counter=0;

                if  minefield[i].chars().nth(j-1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i+1].chars().nth(j).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i+1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
            }else if i==0 {
                let mut counter=0;

                if  minefield[i].chars().nth(j+1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i].chars().nth(j-1).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i+1].chars().nth(j).unwrap() == '*' {
                    counter+=1
                }
                
                if minefield[i+1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i+1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
            }else if i==width-1 {
                let mut counter=0;

                if  minefield[i].chars().nth(j+1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i].chars().nth(j-1).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i-1].chars().nth(j).unwrap() == '*' {
                    counter+=1
                }
                
                if minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i-1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
            }else if j==0 {
                let mut counter=0;

                if  minefield[i+1].chars().nth(j).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i-1].chars().nth(j).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                
                if minefield[i+1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
            }else if j==lenght-1 {
                let mut counter=0;

                if  minefield[i+1].chars().nth(j).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i-1].chars().nth(j).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                
                if minefield[i+1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i-1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
            }else {
                let mut counter=0;

                if  minefield[i].chars().nth(j+1).unwrap() == '*'{ 
                    counter+=1;
                }
                
                if  minefield[i].chars().nth(j-1).unwrap() == '*'{
                    counter+=1;
                }
                
                if minefield[i+1].chars().nth(j).unwrap() == '*' {
                    counter+=1
                }
                
                if minefield[i-1].chars().nth(j).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i+1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i+1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i-1].chars().nth(j-1).unwrap() == '*' {
                    counter+=1
                }
                if minefield[i-1].chars().nth(j+1).unwrap() == '*' {
                    counter+=1
                }
                if counter!=0{
                    let counter_char=(counter + b'0') as char;
                    
                    helper.push(counter_char);
                }else {
                    helper.push('·');
                }
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
        if i != '·' && i != '*' && i != '\n'{
            eprintln!("{}: {} is not valid <'.' for free space and '*' for bombs>","Error".red().bold(),i);
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

    if formatted.len() == 1{
        eprintln!("{}: the minesweeper has to be atleast 2 lines long.","Error".red().bold());
        exit(1);
    }

     
    formatted

}

/*pub fn argument_reading() -> String{

    let mut read = String::new();
    if console::args().count() !=2 {
        eprintln!("{} not the right arguments - build run <minesweeper>;","Error:".bold().red());
        exit(1);
    }

    for i in console::args().skip(1){
        read=i;
    }

    read
} */

fn main() {
    
    let initial: &str = "·*·*·\n··*··\n··*··\n·····";
    let minefield=input_validation(initial);
    let completed=annotate(&minefield);
    
    for i in completed{
        println!("{}",i);
    }
}