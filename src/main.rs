fn count_combinations(n: u32, x: u32) -> u32 {
    if x > n {
        0
    } else {
        (1..=x).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn binom(n:u32, x:u32, p:f32, cumulative: bool, cumulative_type: String){
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
        println!("{result}");
    }
}
fn main() {
    //println!("{}", count_combinations(6, 2));
    //println!("{}", count_permutations(6, 1));
    binom(5,2,0.5,false,"up".to_string());
}
