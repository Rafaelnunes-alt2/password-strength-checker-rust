# password-strength-checker-rust

Analisador de senha em Rust, sem dependência. Calcula entropia em bits,
conta classes de caractere, detecta senha comum e devolve nota de 0 a 5 com
motivo.

## Uso

    cargo install --path .
    pwcheck "Tr0ub4dor&3"

## Como calcula

Entropia = comprimento × log2(tamanho do alfabeto disponível). Pool de 26
para minúsculas, 26 maiúsculas, 10 dígitos, 32 símbolos. Nota sobe com
entropia e cai com senha comum ou curta. Senhas na lista `COMMON` (22
entradas) sempre marcam 0.

## Testes validados

    password marca 0/5 very weak com common password — OK
    abc marca 1/5 weak com too short — OK
    Tr0ub4dor&3 marca 5/5 very strong com 72 bits e 4 classes — OK

## Segurança

A senha nunca é gravada em log, arquivo ou histórico. Só processa o que
recebe em memória e imprime o resultado.

## Limitações

Não checa breach por API, não mede z93 como repetição ou sequencial, não
sugere senha. Modelo simples de entropia aproximada.

## Autor

Rafael Nunes — [@Rafaelnunes-alt2](https://github.com/Rafaelnunes-alt2)