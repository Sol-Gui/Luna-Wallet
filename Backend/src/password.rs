// Função: define senha e criptografa, além de salvar como 
// BLAKE3 PARA CRIPTOGRAFAR A SENHA DA CONTA PRINCIPAL
// ARGON2 PARA CRIPTOGRAFAR A CHAVE PRIVADA (CHAVE PARA ALTERAR A CRIPTOGRAFIA DEVE SER A SENHA DA CONTA PRINCIPAL DESCRIPTOGRAFADA)

use blake3::hash;

pub fn set_password(password: String) {
    let hashed_password = hash(password.as_bytes());
    println!("Hashed Password: {:}", hashed_password.to_hex().to_string());
}