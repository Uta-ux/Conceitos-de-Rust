/*
tipos de dados compostos.
&str - Stirngs (x)
arrays
vetores
tuplas
tuplas vazias
*/

pub fn compostos_datatype(){
    //Strings, temos strings de tamnho fixo e tamnho Flexivel. Exemplo.
    let _nome: &str = "antonio";// tamanho fixo.
    let mut nome2: String = String::from("joao"); // ela se torna Flexivel podendo alterar ela, mas é claro com a condiçao de que ela é um mut.
    nome2.push('s');
    println!("{nome2}");
    //Arrays lista de indices pode ser uma lista de strings ou de Inteiros.
    let lista: [i32;5] = [1 , 2 , 3 , 4 , 5]; // para criar uma lista basicamente precisamos definir antes oq ela vai ser 
    // [i32;5] -> neste caso nos definimos ela como do tipo inteiro 32 bits dps definimos o quanto de espaço aquela lista vai ter

    // e para chamar um indice espercifico precisamos declarar a lista numa posiçao espercificar sendo iqual a variavel que ira criar .

    let resultado = lista[1];

    println!("{resultado}");

    // no terminal vai rodar 2.
    //curiosidade
    let array2: [i32; 10] = [0;10];// basicamente essa array vai cria uma lista com 10 posições com valores zeros.
    let resultado2: i32 = array2[9];
    println!("{resultado2}");

    //vetores - traduzindo arrays Dinamicos para diminuir tamnho, sendo um dos mais utilizados para o desenvolvimentos. podendo definir mudar na hora da compilação sem precisa se preucupar
    let vetor1: Vec<i32> = vec! [1,2,3,4,5,6,7,8,9,10];
    let coisado = vetor1[9];
    println!("{coisado}");
    //tuplas
    let minha_tupla: (&str, &str, &str, i32) = ("nome","antonio","idade",18);

    let nome: &str = minha_tupla.1;

    println!("{nome}");
    
    
}
