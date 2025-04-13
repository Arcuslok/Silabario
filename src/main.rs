use silabario::silabario::*;

fn main() {

    let palabra: &str = "electrodoméstico";
    let silabas: Vec<Vec<char>> = silabizar(palabra);

    println!("{:?}", silabas);

    // Output: [['e', 'l', 'e', 'c'], ['t', 'r', 'o'], ['d', 'o'], ['m', 'é', 's'], ['t', 'i'], ['c', 'o']]

}
