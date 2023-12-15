use std::env;
use std::fs;
use std::vec;

#[derive(Debug)]
struct Instance {
    d: usize,
    p: usize,
    cost_matrix: Vec<Vec<i32>>,
}

fn read_instance(file_path: &String) -> Instance {
    println!("{}", file_path);
    let msg = "Esperando que o arquivo possa ser lido!";
    let contents = fs::read_to_string(file_path).expect(msg);
    
    let mut d: usize = 0;
    let mut p: usize = 0;
    let mut cost_matrix: Vec<Vec<i32>> = vec![];
    let mut line_in_cost_matrix: bool = false;
    let mut line_count_cost_matrix: usize = 0;

    for lines in contents.lines() {    

        if !lines.find("DIMENSION").is_none() {
            let split: Vec<&str> = lines.trim().split_whitespace().collect();
            
            d = split[1].parse().expect("Não foi possivel fazer o parser da dimensão, dimensão tem que estar \"DIMENSION: N\" N sendo um inteiro!!!");
        }

        if !lines.find("P").is_none() {
            let split: Vec<&str> = lines.trim().split_whitespace().collect();
            p = split[1].parse().expect("Não foi possivel fazer o parser da restrição P, P tem que estar \"P: M\" M tem que ser um inteiro!!!")
        }

        if !lines.find("COST").is_none() {
            line_in_cost_matrix = true;
        }

        if line_in_cost_matrix {

            if line_count_cost_matrix == 0 {
                line_count_cost_matrix += 1;
                continue;
            }
            
            let mut vector: Vec<Vec<i32>> = vec![lines.trim().split_whitespace().map(|l| l.parse().unwrap()).collect()];
            
            cost_matrix.append(&mut vector);

            if line_count_cost_matrix >= d {
                break;
            }

            line_count_cost_matrix += 1;

        }

    }

    Instance{
        d, 
        p, 
        cost_matrix
    }
    
    
}

fn nearest_neighbor(instance: &Instance) {
    
    let all_vertices_visited = false;
    let count_visited_vertices: u32 = 0;
    let mut visited_vertices = vec![false; instance.d];
    visited_vertices[0] = true;

    let mut iter: u32 = 0;
    let mut current_vertex: u32 = 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
   
    if args.len() > 1 {
        
        let file_path = &args[1];
        let instance = read_instance(file_path);

        println!("{:?}", instance);
    }else{
        println!("Necessario passar uma instancia para o programa.");
    }

}
