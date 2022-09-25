## TSP in Rust ##
Este repositório contém uma implementação do algoritmo de força bruta O(n!)
para encontrar a solução exata para o problema do caixeiro viajante (Traveling Salesman Problem)
Foi feito como exercício para aprendizado da linguagem Rust, e comparação com a versão em C

## Algorimto ##
O algoritmo utiliza permutações em ordem lexicográfica dos vértices para testar todos os caminhos,
testa (n-1)! possibilidades, sendo n o número de vértices do grafo

## Benchmark ##
Em questão de tempo, para a entrada de 15 vértices (.\src\data_set\tsp3_1194.csv)
este código em Rust terminou em 19 minutos enquanto a versão em C levou 29 minutos
Testado em um R5 5600x

## Build and Run ##
Para rodar, tenha o compilador rust instalado e faça:
1º clone o repositório -> git clone https://github.com/JelsonRodrigues/tsp_in_rust.git
2º entre na pasta do repositório -> cd tsp_in_rust
3º compile o programa -> cargo build --release
4º rode o programa -> (windows) ./target/release/tsp_in_rust.exe .\src\data_set\tsp1_253.csv
                   -> (linux)   ./target/release/tsp_in_rust.o .\src\data_set\tsp1_253.csv

## Observações ##
Após compilado o programa, você pode escolher o arquivo de entrada passando o nome do arquivo
após o nome do binário executável na linha de comando. Existem 5 arquivos com matrizes de adjacência na pasta
./src/data_set/ tenha em mente apenas que a complexidade de tempo do algoritmo é fatorial O(n!) e 
é impossível rodar até o final para as entradas .\src\data_set\tsp4_7013 e .\src\data_set\tsp5_27603.csv
