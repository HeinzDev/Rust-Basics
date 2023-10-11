use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn read_file(filename: &str) -> Result<Vec<i32>, io::Error> {
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;
    
    let splitted: Result<Vec<i32>, ParseIntError> = content
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect();

    match splitted { 
        Ok(numbers) => Ok(numbers),
        Err(parse_error) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Erro na conversão: {}", parse_error),
        )),
    }
}

fn swap_array(lista: &mut Vec<i32>, i: usize, j: usize) {
    let temp: i32 = lista[i];
    lista[i] = lista[j];
    lista[j] = temp;
}

fn main() {
    let filename = String::from("file.txt");
    let mut array = match read_file(&filename) {
        Ok(arr) => arr,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo: {}", err);
            return;
        }
    };

    for i in 0..array.len() {
        for j in ((i + 1)..array.len()).rev() {
            if array[j - 1] > array[j] {
                swap_array(&mut array, j - 1, j);
            }
        }
    }

    println!("{:?}", array);
}
