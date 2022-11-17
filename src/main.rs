use std::env;
use std::process;

fn count_combinations(n: u32, x: u32) -> u32 {
    if x > n {
        0
    } else {
        (1..=x).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn binom(n:u32, x:u32, p:f32, cumulative: bool, cumulative_type: &String){
    if cumulative == true{
        match &cumulative_type[..]{
            "up" => {
                let mut x = x as f32;
                let mut cumulative_value: Vec<f32>= Vec::new();
                while x > -1.0{
                    let get_combinations = count_combinations(n,x as u32);
                    let result = get_combinations as f32 *p.powf(x as f32)*(1 as f32 -p).powf(n as f32-x as f32);
                    cumulative_value.push(result);
                    x -= 1.0;
                }
                let cumulate_all_value:f32 = cumulative_value.iter().sum();
                let cumulate_all_value:f32 = 1.0 -cumulate_all_value;
                if cumulate_all_value > 1.0 {
                    println!("1");
                    return;
                }
                else if cumulate_all_value <= -0.0 {
                    println!("0");
                    return;
                }
                println!("{}",cumulate_all_value);
                }
            "down" => {
                let mut x = x as f32;
                let mut cumulative_value: Vec<f32>= Vec::new();
                while x > -1.0{
                    let get_combinations = count_combinations(n,x as u32);
                    let result = get_combinations as f32 *p.powf(x as f32)*(1 as f32 -p).powf(n as f32-x as f32);
                    cumulative_value.push(result);
                    x -= 1.0;
                }
                let cumulate_all_value:f32 = cumulative_value.iter().sum();
                if cumulate_all_value > 1.0 {
                    println!("1");
                    return;
                }
                else if cumulate_all_value <= -0.0 {
                    println!("0");
                    return;
                }
                println!("{}",cumulate_all_value);
            }
            _ => {
                println!("Unknown command!");
            }
        }
    }
    else {
        let get_combinations = count_combinations(n,x);
        let result = get_combinations as f32 *p.powf(x as f32)*(1 as f32 -p).powf(n as f32-x as f32);
        if result > 1.0 {
            println!("1");
            return;
        }
        else if result <= -0.0 {
            println!("0");
            return;
        }
        println!("{result}");
    }
}

fn print_help(){
    println!("Usage :\n arguments: n , x , p, cumulative, up/down[depend]");
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let help_args = vec!["-h".to_string(), "--help".to_string()];
    if help_args.iter().any(|e| args.contains(e)){
        print_help();
        process::exit(0)
    }

    if args.len() < 5 {
        println!("Not enought arguments!");
        print_help();
        process::exit(1);
    }

    let cumulative: bool = args[4].parse().unwrap_or_else(|e| {
            println!("'cumulative' variable must be a bool!\n {e}");
            print_help();
            process::exit(1);
        }); 

    if cumulative == false { 
        let n:u32 = args[1].parse().unwrap_or_else(|e| {
            println!("'n' variable must be an u32!\n {e}");
            print_help();
            process::exit(1);
        });

        let x:u32 = args[2].parse().unwrap_or_else(|e| {
            println!("'x' variable must be an u32!\n {e}");
            print_help();
            process::exit(1);
        });

        let p:f32 = args[3].parse().unwrap_or_else(|e| {
            println!("'p' variable must be a f32!\n {e}");
            print_help();
            process::exit(1);
        });
        if p > 1.0 {
            println!("'p' cant be more than 1.0!");
            process::exit(1);
        }
        let cumulative_string: String = "none".to_string();
        args.push(cumulative_string);
        binom(n,x,p,cumulative,&args[5]);
    }

    else {
        if args.len() < 6 {
            println!("Not enought arguments!");
            print_help();
            process::exit(1);
        }
        let n:u32 = args[1].parse().unwrap_or_else(|e| {
            println!("'n' variable must be an u32!\n {e}");
            print_help();
            process::exit(1);
        });

        let x:u32 = args[2].parse().unwrap_or_else(|e| {
            println!("'x' variable must be an u32!\n {e}");
            print_help();
            process::exit(1);
        });

        let p:f32 = args[3].parse().unwrap_or_else(|e| {
            println!("'p' variable must be a f32!\n {e}");
            print_help();
            process::exit(1);
        });
        if p > 1.0 {
            println!("'p' cant be more than 1.0!");
            process::exit(1);
        }
        binom(n,x,p,cumulative,&args[5]);
    }
}
