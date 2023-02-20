use clap::Parser;
use std::borrow::Borrow;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{channel, Sender};
use std::thread::{available_parallelism, JoinHandle};

// Passagem de argumentos utilizando a crate clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Caminho do arquivo com matriz de adjacência no formato .csv separado por ';'
    #[arg(short, long)]
    caminho_arquivo_grafo: String,

    /// Número de threads para utilizar no programa (0 todas)
    #[arg(short, long, default_value_t = 0)]
    numero_threads: usize,
}

fn main() {
    // Pega os arqumentos passados por linha de comando
    let args = Args::parse();

    // Lê o grafo
    let grafo = ler_arquivo(args.caminho_arquivo_grafo.borrow());
    let grafo = Box::new(grafo);

    let numero_threads = {
        let total_threads = available_parallelism().unwrap().get();
        if total_threads < args.numero_threads || args.numero_threads == 0 {
            total_threads
        } else {
            args.numero_threads
        }
    };

    // Variáveis com os resultados finais
    let mut minimo = u32::MAX;
    let mut indice_minimo = 0;
    let mut maximo = u32::MIN;
    let mut indice_maximo = 0;

    // Mede o tempo
    let antes = std::time::Instant::now();

    // Multithread stuff
    println!("Utilizando {numero_threads} threads");
    let numero_permutacoes_por_thread = calcula_fatorial(grafo.len() - 1) / numero_threads;
    let (tx, rx) = channel::<(u32, u64)>();

    // Criacao das threads
    for i in 0..numero_threads {
        let indice_primeira_permutacao = i * numero_permutacoes_por_thread;
        let indice_ultima_permutacao = (i + 1) * numero_permutacoes_por_thread;
        let grafo = grafo.clone();
        let transmitter_end = tx.clone();
        std::thread::spawn(move || {
            tsp_forca_bruta(
                grafo,
                indice_primeira_permutacao as u64,
                indice_ultima_permutacao as u64,
                transmitter_end,
            );
        });
    }

    drop(tx);

    while let Ok(valor) = rx.recv() {
        if valor.0 < minimo {
            minimo = valor.0;
            indice_minimo = valor.1;
            print!("\rMenor valor {minimo}\tMaior valor {maximo}");
        } else if valor.0 > maximo {
            maximo = valor.0;
            indice_maximo = valor.1;
            print!("\rMenor valor {minimo}\tMaior valor {maximo}");
        }
    }

    let agora = std::time::Instant::now();

    // Mostra o peso e o caminho
    println!();
    println!("\tMenor caminho: {minimo}");
    imprime_ordem_lexicografica_n(indice_minimo, grafo.len());
    println!("\tMaior caminho: {maximo}");
    imprime_ordem_lexicografica_n(indice_maximo, grafo.len());

    // Mostra o tempo
    let tempo_decorrido_ms = (agora - antes).as_millis() % 1000;
    let tempo_decorrido_s = (agora - antes).as_secs() % 60;
    let tempo_decorrido_min = (agora - antes).as_secs() / 60 % 60;
    let tempo_decorrido_hora = (agora - antes).as_secs() / 60 / 60;
    println!("\tTempo {tempo_decorrido_hora}h:{tempo_decorrido_min}m:{tempo_decorrido_s}s:{tempo_decorrido_ms}ms");
    println!("Numero de threads utilizadas: {numero_threads}")
}

fn imprime_grafo(grafo: &Vec<Vec<u32>>) {
    for item in grafo {
        println!("{:?}", item);
    }
}

fn imprime_ordem_lexicografica_n(n: u64, numero_elementos: usize) {
    let mut vetor: Vec<usize> = (0..numero_elementos).collect();

    n_esima_ordem_lexicografica(&mut vetor, n);

    println!("{:?}", vetor);
}

fn ler_arquivo(caminho: &str) -> Vec<Vec<u32>> {
    let arquivo = std::fs::File::open(caminho).expect("Não foi possível abrir o arquivo\n");

    let mut buf_reader = BufReader::new(arquivo);
    let mut linha = String::new();

    let mut grafo: Vec<Vec<u32>> = Vec::new();

    while buf_reader.read_line(&mut linha).unwrap() > 0 {
        // Remove o \n do final da linha, para não ocorrer erro com o split
        if linha.ends_with("\n") {
            linha.pop();
        }

        // Adiciona uma nova linha no grafo
        grafo.push(Vec::new());

        // Separa a linha no delimitador de csv ';'
        for coluna in linha.split(';') {
            let valor = coluna.parse::<u32>();
            if valor.is_ok() {
                grafo.last_mut().unwrap().push(valor.unwrap());
            }
        }

        linha.clear();
    }

    return grafo;
}

fn proxima_ordem_lexicografica(vetor: &mut Vec<usize>) -> bool {
    if vetor.len() <= 1 {
        return false;
    }

    let mut i = vetor.len() - 2;
    let mut j = vetor.len() - 1;

    while (i > 0) && (vetor[i] > vetor[i + 1]) {
        i -= 1;
    }
    while (j > 0) && (vetor[j] < vetor[i]) {
        j -= 1;
    }

    if i != j {
        vetor.swap(i, j);
        let pedaco_inverter = &mut vetor[i + 1..];
        pedaco_inverter.reverse();
        return true;
    }

    return false;
}

fn tsp_forca_bruta(
    grafo: Box<Vec<Vec<u32>>>,
    indice_primeira_permutacao: u64,
    indice_ultima_permutacao: u64,
    transmitter: Sender<(u32, u64)>,
) -> ((u32, u64), (u32, u64)) {
    let mut min: u32 = std::u32::MAX;
    let mut indice_min: u64 = 0;
    let mut max: u32 = std::u32::MIN;
    let mut indice_max: u64 = 0;
    let mut peso_caminho_atual: u32;

    let mut permutacao_atual = indice_primeira_permutacao;

    // Cria o vetor com a permutação atual, contém o índice do vértice
    // na matriz
    let mut caminho_atual: Vec<usize> = (0..grafo.len()).collect();

    // Pega a n-ésima ordem lexicográfica
    n_esima_ordem_lexicografica(&mut caminho_atual, indice_primeira_permutacao);

    // Calcular o peso e atualiza a mínima e máxima
    while permutacao_atual < indice_ultima_permutacao {
        peso_caminho_atual = 0;
        (0..caminho_atual.len()).for_each(|indice| {
            peso_caminho_atual +=
                grafo[caminho_atual[indice]][caminho_atual[(indice + 1) % caminho_atual.len()]];
        });

        if peso_caminho_atual < min {
            indice_min = permutacao_atual;
            min = peso_caminho_atual;
            transmitter.send((min, indice_min)).unwrap();
        }

        if peso_caminho_atual > max {
            indice_max = permutacao_atual;
            max = peso_caminho_atual;
            transmitter.send((max, indice_max)).unwrap();
        }

        permutacao_atual += 1;

        if proxima_ordem_lexicografica(&mut caminho_atual) == false {
            break;
        }
    }

    return ((min, indice_min), (max, indice_max));
}

fn n_esima_ordem_lexicografica(vetor: &mut Vec<usize>, n: u64) {
    // https://ichi.pro/pt/permutacoes-eficientes-em-ordem-lexicografica-156131986289848

    if vetor.len() < 1 {
        return;
    }

    // Calcula fatorial de (numero de itens - 1)
    let mut fatorial: usize = calcula_fatorial(vetor.len() - 1);

    // Verifica se a posição lexicográfica existe para o tamanho do vetor
    if n >= (fatorial * vetor.len()).try_into().unwrap() {
        return;
    }

    let mut resto_anterior: usize = n.try_into().unwrap();
    let mut indice_proximo: usize;

    // Cria uma cópia do vetor original, deixa ordenada (1ª ordem lexicográfica)
    // e limpa o vetor original
    let mut indices: Vec<usize> = vetor.clone();
    indices.sort();
    vetor.clear();

    while (indices.len() > 0) && fatorial > 0 {
        // Calcula o índice do próximo item
        indice_proximo = resto_anterior / fatorial;
        resto_anterior = resto_anterior % fatorial;

        // Aqui é calculado o fatorial de indices.len() - 1
        if indices.len() - 1 > 0 {
            fatorial = fatorial / (indices.len() - 1);
        } else {
            fatorial = 1;
        }

        // Retira dos vetor indices o item calculado e adiciona no final vetor de resultado
        vetor.push(indices.remove(indice_proximo));
    }
    return;
}

fn calcula_fatorial(numero: usize) -> usize {
    let mut resultado: usize = 1;
    let mut numero = numero;
    if numero > 1 {
        while numero > 0 {
            resultado *= numero;
            numero -= 1;
        }
    }
    return resultado;
}
