/*
* tipo de dados basicos.
*
* Integers
* Floats
* Chars
* Boolean.
*/ 
// vamos começa escrevendo uma Função.
// explicação do que é inferencia de tipo é quando o sistema defini por padrão um "tipo de alguma coisa".


pub fn _data_types(){
    //Unsigner Integers.
    let _unsigner_num: u8 = 5; // tipos de bits, u8,u16,u32,u64,u128. 8bits - 1byte, 16bits - 2
    // bytes, 32bits - 4 bytes, 64bits - 8 bytes, 128bits - 16bytes.

    //Signed Integers.
    let _signed_num: i8 = 10;  // tipos de bits, i8, i16, i32,i64,i128,-> ele pode os dois tipos de
    // Numero negativo e positivo.
    //
    // alocaçao de inteiros com base na arquitetura.
    let _arch1: usize = 5;
    //signed
    let _arch2: isize = 10;
    // char. para uma letra só literalmente.
    let _caractere: char = 'a';
    // boolean. para verdadeiro ou falso.
     let _bolean: bool = false;// true
    //alias transformação de bits em variaveis.
    type Numero = u8;
    let _idade_joao: Numero = 18;

    //Conversao de dados
    let a: f32 = 3.64;
    let _b: i32 = a as i32;

    //print disso vai ser 3, e isso é usado muito.

    






}


pub fn matematica(a:i32, b:i32 )->i32{
    a + b
}
