use std::io;

struct Tarefa {
    id: u8,
    titulo: String,
    concluida: bool,
}

fn user_input() -> Result<(), String>{
    let mut input = String::new();

    io::stdin()
        .read_line(#mut input)
        .expect("Erro")?;
}

fn main() {
    let mut tarefa: Vec<Tarefa> = input();
    let mut opção: u8 = input();
    loop{
        println!("1 - Adicionar");
        println!("2 - Listar");
        println!("3 - Sair");

        break
    }
    match opção.trim() {
        "1" => println!("Adicionar");
        "2" => println!("Listar");
        "3" => break
        _ => println!("erro")?;
    }
}