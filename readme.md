## **TSP in Rust** ##

Este repositório contém uma implementação do algoritmo de força bruta multithread **O(n!)**
para encontrar a solução exata para o problema do caixeiro viajante (Traveling Salesman Problem)<br>
Foi feito como exercício para aprendizado da linguagem **Rust**, e comparação com a versão em **C**

## Algorimto ##

O algoritmo utiliza permutações em ordem lexicográfica dos vértices para testar todos os caminhos,
testa **(n-1)!** possibilidades, sendo **n** o número de vértices do grafo.

## Benchmark ##

Em questão de tempo, para a entrada de 15 vértices `(.\src\data_set\tsp3_1194.csv)`
este código em **Rust** terminou em **20 minutos** enquanto a versão em **C** levou **29 minutos**<br>
Testado em um R5 5600x que possui 6 cores e 12 threads<br>
Threads <br>
1 -> 20m33s<br>
2 -> 11m32s<br>
4 -> 05m16s<br>
6 -> 03m43s<br>
8 -> 04m01s<br>
12 -> 03m38s<br>

## Build and Run ##

Para rodar, tenha o compilador rust e o cargo instalado e faça:<br>
1º clone o repositório -> `git clone https://github.com/JelsonRodrigues/tsp_in_rust.git`<br>
2º entre na pasta do repositório -> `cd tsp_in_rust`<br>
3º compile o programa -> `cargo build --release`<br>
4º rode o programa <br>-> (*windows*) `./target/release/tsp_in_rust.exe -c .\src\data_set\tsp1_253.csv`<br>
                   -> (*linux*)   `./target/release/tsp_in_rust -c .\src\data_set\tsp1_253.csv`<br>

## Observações ##

Após compilado o programa, você pode escolher o arquivo de entrada passando o nome do arquivo como parâmetro `-c` ou 
`--caminho-arquivo-grafo` após o nome do binário executável na linha de comando. Existem 5 arquivos com matrizes de adjacência na pasta
`./src/data_set/` tenha em mente apenas que a complexidade de tempo do algoritmo é fatorial **O(n!)** e 
é **impossível** rodar até o final para as entradas `.\src\data_set\tsp4_7013` e `.\src\data_set\tsp5_27603.csv`<br>
É possível também especificar o número de threads a ser utilizado, passando após o nome do arquivo de entrada o parâmetro 
`-n <NUMERO_THREADS>` ou `--numero-threads <NUMERO_THREADS>`, caso o valor passado seja maior que o total de threads no seu 
sistema, então serão utilizado o maximo disponível. Se não for especificado nenhum valor, o máximo de threads será utilizado.
