use std::borrow::Borrow;
use std::io::{BufReader, BufRead};
use std::env;
use std::process::exit;

fn main() {
    // Pega os arqumentos passados por linha de comando
    let argumentos: Vec<String> = env::args().collect();
    if argumentos.len() < 2 {
        println!("Passe o nome do arquivo com a matriz de adjacência como parâmetro");
        exit(-1);
    }
    
    // Tenta ler o arquivo
    let grafo = ler_arquivo(argumentos[1].borrow());
    
    // Mede o tempo e calcula o menor e maior caminho
    let antes = std::time::Instant::now();
    let ((minimo, indice_minimo),(maximo, indice_maximo)) = tsp_forca_bruta(&grafo, 0, (calcula_fatorial(grafo.len() - 1) - 1).try_into().unwrap());
    let agora = std::time::Instant::now();
    
    // Mostra o peso e o menor caminho
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
}

fn imprime_grafo(grafo : &Vec<Vec<u32>>){
    for (i, linha) in grafo.iter().enumerate() {
        for (j, coluna) in linha.iter().enumerate() {
            print!("({i}, {j}) : {coluna}\t");
        }
        println!();
    }
}

fn imprime_ordem_lexicografica_n(n : u64, numero_elementos : usize){
    let mut vetor:Vec<usize> = Vec::new();
    for numero in 0..numero_elementos {
        vetor.push(numero);
    }

    n_esima_ordem_lexicografica(&mut vetor, n);

    for item in vetor {
        print!("{item} -> ");
    }
    println!();
}

fn ler_arquivo(caminho : &str) -> Vec<Vec<u32>>{
    let arquivo = std::fs::File::open(caminho)
                                        .expect("Não foi possível abrir o arquivo\n");
    
    let mut buf_reader = BufReader::new(arquivo);
    let mut linha = String::new();
    
    let mut grafo:Vec<Vec<u32>> = Vec::new();

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

fn proxima_ordem_lexicografica(vetor: &mut Vec<usize>) -> bool{
    
    if vetor.len() <= 1 {return  false;}

    let mut i = vetor.len() - 2;
    let mut j = vetor.len() - 1;

    while (i > 0) && (vetor[i] > vetor[i + 1]){i-=1;};
    while (j > 0) && (vetor[j] < vetor[i]) {j-=1;};

    if i != j {
        vetor.swap(i, j);
        let pedaco_inverter = &mut vetor[i+1..];
        pedaco_inverter.reverse();
        return true;
    }

    return  false;
}

fn tsp_forca_bruta(grafo : &Vec<Vec<u32>>, indice_primeira_permutacao :u64, indice_ultima_permutacao : u64) -> ((u32, u64), (u32, u64)){
    let mut min: u32 = std::u32::MAX;
    let mut indice_min: u64 = 0;
    let mut max: u32 = std::u32::MIN;
    let mut indice_max: u64 = 0;
    let mut peso_caminho_atual: u32;

    let mut permutacao_atual = indice_primeira_permutacao;
    
    // Cria o vetor com a permutação atual, contém o índice do vértice
    // na matriz
    let mut caminho_atual: Vec<usize> = Vec::new();
    for numero in (0 .. grafo.len()).into_iter() {
        caminho_atual.push(numero);
    }

    // Pega a n-ésima ordem lexicográfica
    n_esima_ordem_lexicografica(&mut caminho_atual, indice_primeira_permutacao);

    // Calcular o peso e atualiza a mínima e máxima
    while permutacao_atual < indice_ultima_permutacao {
        peso_caminho_atual = 0;
        for indice in 0 .. caminho_atual.len() {
            peso_caminho_atual += grafo[caminho_atual[indice]][caminho_atual[(indice + 1) % caminho_atual.len()]];
        }

        if peso_caminho_atual < min{
            indice_min = permutacao_atual;
            min = peso_caminho_atual;
            println!("Menor PESO: {min}");
        }
        
        if peso_caminho_atual > max{
            indice_max = permutacao_atual;
            max = peso_caminho_atual;
            println!("Maior PESO: {max}");
        }

        permutacao_atual += 1;
        
        if proxima_ordem_lexicografica(&mut caminho_atual) == false {break;}
    }

    return ((min, indice_min), (max, indice_max));
}


fn n_esima_ordem_lexicografica(vetor: &mut Vec<usize>, n: u64){
    // https://ichi.pro/pt/permutacoes-eficientes-em-ordem-lexicografica-156131986289848
    
    if vetor.len() < 1 {return;}

    // Calcula fatorial de (numero de itens - 1)
    let mut fatorial : usize = calcula_fatorial(vetor.len() - 1);

    // Verifica se a posição lexicográfica existe para o tamanho do vetor
    if n >= (fatorial * vetor.len()).try_into().unwrap() {return;}

    let mut resto_anterior : usize = n.try_into().unwrap();
    let mut indice_proximo : usize;

    // Cria uma cópia do vetor original, deixa ordenada (1ª ordem lexicográfica)
    // e limpa o vetor original
    let mut indices:Vec<usize> = vetor.clone();
    indices.sort();
    vetor.clear();
    
    while (indices.len() > 0) && fatorial > 0 {
        // Calcula o índice do próximo item 
        indice_proximo = resto_anterior / fatorial;
        resto_anterior = resto_anterior % fatorial;
        
        // Aqui é calculado o fatorial de indices.len() - 1
        if indices.len() - 1 > 0 {
            fatorial = fatorial / (indices.len() - 1);
        }
        else {
            fatorial = 1;
        }

        // Retira dos vetor indices o item calculado e adiciona no final vetor de resultado
        vetor.push(indices.remove(indice_proximo));
    }
    return;
}

fn calcula_fatorial(numero : usize) -> usize {
    let mut resultado : usize = 1;
    let mut numero = numero;
    if numero > 1 {
        while numero > 0 {
            resultado *= numero;
            numero -= 1;
        }
    }
    return  resultado;
}
