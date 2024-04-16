use std::process::exit;

use colored::*;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    
    let completed: Vec<String> = Vec::new();
    
    let width=minefield.len();
    let lenght:u64=lenght_calculator(minefield[0]);
    
    for i in 0..width{
        for j in 0..lenght{
            print!("|");
            todo!("implement bomb replace logic using comment");
        }
        println!();
    }
    //print!("{:?}",_minefield[0].chars().nth(2));
    
    print!("width:{}, length:{}",width,lenght);

    completed
}

pub fn lenght_calculator(row: &str) -> u64{
   
    let mut lenght:u64=0;
    
    for _i in row.chars(){
        lenght+=1;
    }

    lenght
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

    let lenght=lenght_calculator(formatted[0]);

    for i in &formatted{
        let verifier=lenght_calculator(i);

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

pub fn argument_validation() -> &str{

    let arg: &str="";
    
    arg
}

fn main() {
    
    let initial: &str = "·*·*·\n··*··\n··*··\n·····";

    let minefield=input_validation(initial);

    println!("{:?}",minefield);
    annotate(&minefield);
}