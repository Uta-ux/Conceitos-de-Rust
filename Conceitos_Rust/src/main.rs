mod dado_compostos;
mod dados_primitivos;

fn main() {

    //variaveis para aprender.
    //vamo começa com inteiros
    let mut x: i32 = 10; // agr ele se tornou uma variavel mutavel agora eu posso bota outro valor
    // que ele vai adquirir :)
    let y: u32 = 20; // unsigned integer, ele não pode retroceder para Numeros negativos.

    let z: f64 = 30.5; // float para numeros quebrados

    let name: String = String::from("Joao");// estamos transformando uma varialvel em String,
        // Referenciando chamando para um Struck de String. // e isso so é um dos jeitos temos mais
        // dois.
        let _name2: &str = "joao supremo"; // Profissional e respeitoso.

    println!("valor de inteiro normal {x}, valor de inteiro que não recebe numeros negativos é {y}, e numeros de float que sao numeros quebrados é {z}");

    
    // mutabilidades

    x = 11;
        println!("o novo valor de x é {x}");

    println!("o nosso nome é {name}");

{
        let name = "Antonio";
        println!("nome dentro do Escopo é : {name}");
        let base_calculo: i32 = 2;

        x = 1000 * base_calculo;
    }

    println!("meu nome é {name}");

    println!("novo valor atribuido de outro escopo, {x}");

    //shadowing - imutavel.
    let _idade: i32 = 37; 
    // idade = 16 ia dar erro pelo simples fato que a variavel idade não é mutavel.
    let idade: i32 = 16;

    println!("a idade é : {idade}");

    //constantes.
    const TENTATIVAS: i8 = 10;
    println!("TENTATIVAS é : {TENTATIVAS}");

    // vairiavel tipo: char.
    let primeira_letra: char = 'a';
    println!("primeira letra da String, é : {primeira_letra}");

    let x = 10;
    let y = 5;

    let total = dados_primitivos::matematica(x,y);

    println!("o resultado é: {}",total);

    dado_compostos::compostos_datatype(); // nome_arquivo::funçao que estar dentro do arquivo.

    let _celsius: f64 = 34.9;

    let fire: f64 = _celsius * 9.0 / 5.0 + 32.0;
    println!("os graus são {} ",fire);
    let _kelvin: f64 = 34.9 + 273.15;
    println!("a quantidade de kelvins sao, {} ",_kelvin);
    //testando arredondamento de resposta.
    let _kj: i128 = _kelvin as i128;
    println!("numero talvez arredondado, {_kj}");

    let _tup:(f64, f64, f64) = (_celsius,fire,_kelvin);
    //para printar uma tupla vc precisa usar dentro do print {:?}
    println!("o resultado de todas as temperaturas sao {:?}",_tup);
}

