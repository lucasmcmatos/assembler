# Assembler Hack — Nand2Tetris Project 06

- **Aluno:** Lucas Martins Campos Matos
- **RA:** 20250013668
- **Disciplina:** Compiladores
- **Linguagem:** Rust (edition 2021)
- **Link do Vídeo:** https://youtu.be/6IF6XoKF4Rc
## Descrição

Assembler para a linguagem Assembly Hack (Nand2Tetris, Projeto 06). Traduz arquivos
`.asm` em código de máquina binário de 16 bits (`.hack`), em duas passagens:

1. **Primeira passagem**: percorre o arquivo e registra os labels (`(LABEL)`) na tabela
   de símbolos, com o endereço da próxima instrução real.
2. **Segunda passagem**: resolve cada A-instruction (constante numérica, símbolo já
   conhecido ou nova variável a partir do endereço 16) e monta o código binário de
   cada C-instruction (`111` + `comp` + `dest` + `jump`), escrevendo o resultado em
   `<nome>.hack`.

## Estrutura do projeto

```
src/
├── main.rs          # orquestrador (primeira e segunda passagens, escrita do .hack)
├── parser.rs         # leitura do .asm, remoção de comentários/espaços, tokenização
├── symbol_table.rs    # tabela de símbolos (predefinidos + labels + variáveis)
└── code.rs            # tabelas de encoding (comp/dest/jump)
tests/
└── integration_tests.rs   # roda o binário contra add/max/maxL/rect/rectL.asm
test_files/            # arquivos oficiais do Nand2Tetris usados nos testes
```

## Build

```bash
cargo build --release
```

## Execução

```bash
cargo run --release -- test_files/add.asm
# gera test_files/add.hack
```

## Testes

```bash
# testes unitários (parser, symbol_table, code, main) + integração (add/max/maxL/rect/rectL)
cargo test
```

## Lint e formatação

```bash
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Exemplos de uso

```bash
cargo build --release

./target/release/assembler test_files/add.asm
cat test_files/add.hack
# 0000000000000010
# 1110110000010000
# 0000000000000011
# 1110000010010000
# 0000000000000000
# 1110001100001000

./target/release/assembler test_files/max.asm
./target/release/assembler test_files/rect.asm
./target/release/assembler test_files/pong.asm
```

## Vídeo de demonstração

_(link a preencher)_
