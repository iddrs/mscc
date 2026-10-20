# MSC Converter — GUI (Iced/Rust)

Interface gráfica (visual dark) para executar o conversor

```text
php mscc.php <source_dir> <output_file>
```

A GUI possui:

- Cabeçalho com o nome e a função do programa;
- Campo **Diretório da MSC** (argumento `source_dir`) com botão "Procurar…";
- Campo **Arquivo DuckDB** (argumento `output_file`) com botão "Procurar…"
  — o diálogo permite selecionar um arquivo que ainda **não existe**;
- Botão **Executar** (mais *Cancelar* e *Limpar saída*);
- Área com scrollbar que exibe a saída *raw* do script PHP em tempo real
  (stdout + stderr, com erros destacados em vermelho);
- Barra de status com as informações do desenvolvedor;
- **Memória dos caminhos**: os valores de `source_dir` e `output_file` são
  salvos automaticamente num arquivo `mscc-gui.ini` ao lado do executável e
  recarregados na próxima inicialização (argumentos da linha de comando têm
  prioridade sobre os valores salvos).

O `mscc.php` é localizado automaticamente ao lado do executável ou no
diretório atual; o processo PHP é executado com o diretório de trabalho do
script (para que `sql/*.sql` e `external_data.xlsx` sejam resolvidos
corretamente).

## Requisitos

- Rust (estável) — <https://rustup.rs>
- PHP 8.5+ no `PATH` (o mesmo usado pelo `mscc.php`);
- No Windows, a toolchain precisa de um linker: MSVC (Visual Studio Build
  Tools) ou MinGW-w64 (`gcc`).

## Compilar (recomendado: release)

```powershell
cd gui
cargo build --release
```

O executável fica em `gui\target\release\mscc-gui.exe`.

## Executar

```powershell
# A partir do diretório do projeto (para que o mscc.php seja encontrado):
.\gui\target\release\mscc-gui.exe

# Opcionalmente, pré-preenchendo os argumentos:
.\gui\target\release\mscc-gui.exe "C:\dados\msc" "C:\dados\msc.duckdb"
```

Para desenvolvimento, com hot reload (requires `cargo-watch`):

```powershell
cargo install cargo-watch
cargo watch -x run
```
