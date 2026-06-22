use std::io;

struct Tarefa {
    id: u8,
    titulo: String,
    concluida: bool,
}

fn user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro");
    input
}

fn main() {
    let mut tarefas: Vec<Tarefa> = Vec::new();
        loop{
            println!("1 - Adicionar");
            println!("2 - Listar");
            println!("3 - Sair");
            
            let opção = user_input();

            match opção.trim() {
                "1" => {
                    println!("Nome da tarefa:");
                    let nome: String = user_input();
                    println!("Concluída: s/n");
                    let conclusao = user_input();
                    let c_value: bool = match conclusao.trim() {
                        "s" => true,
                        "n" => false,
                        _ => {
                            println!("Considerado falso.");
                            false
                        },
                    };
                    let work = Tarefa{
                        id: 0,
                        titulo: nome,
                        concluida: c_value,
                    };
                    tarefas.push(work);
                },
                "2" => { 
                    if tarefas.is_empty() {
                        println!("Nenhuma tarefa.")
                    } else {
                        for work in &tarefas{
                            if work.concluida{
                                println!("[X] {}", work.titulo);
                            } else {
                                println!("[ ] {}", work.titulo);
                            }
                        }
                    }
                },
                "3" => {
                    println!("Saindo...");
                    break
                },
            _ => println!("erro"),
            }
        }
        }