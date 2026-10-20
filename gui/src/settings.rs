//! Persistência dos caminhos informados na GUI.
//!
//! Os valores de `source_dir` e `output_file` são gravados num pequeno
//! arquivo `mscc-gui.ini` ao lado do executável (ou, como fallback, no
//! diretório atual) e recarregados na próxima inicialização.
//!
//! O formato é deliberadamente simples (sem dependências externas):
//!
//! ```text
//! # MSC Converter — caminhos usados na última execução
//! source_dir=C:\dados\msc
//! output_file=C:\dados\msc.duckdb
//! ```
//!
//! Todas as operações são *best-effort*: falhas de leitura/gravação são
//! silenciosamente ignoradas e nunca impedem o funcionamento da GUI.

use std::path::{Path, PathBuf};

/// Nome do arquivo de configuração.
pub const FILE_NAME: &str = "mscc-gui.ini";

/// Caminhos informados pelo usuário, mantidos entre execuções.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Settings {
    pub source_dir: String,
    pub output_file: String,
}

/// Localiza o arquivo de configuração: ao lado do executável ou no
/// diretório atual (fallback).
pub fn path() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        return Some(dir.join(FILE_NAME));
    }

    std::env::current_dir().ok().map(|dir| dir.join(FILE_NAME))
}

/// Carrega as configurações salvas (silenciosamente vazio se não houver).
pub fn load() -> Settings {
    match path() {
        Some(path) => load_from(&path),
        None => Settings::default(),
    }
}

/// Grava as configurações atuais (silencioso em caso de falha).
pub fn save(settings: &Settings) {
    if let Some(path) = path() {
        save_to(&path, settings);
    }
}

/// Lê o arquivo no formato `chave=valor` (linhas vazias e `#` são ignoradas).
fn load_from(path: &Path) -> Settings {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Settings::default();
    };

    let mut settings = Settings::default();

    for line in content.lines() {
        let line = line.trim_start();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Divide só no primeiro '=' — caminhos podem conter '='.
        if let Some((key, value)) = line.split_once('=') {
            // `trim_end` cobre eventuais '\r' de arquivos editados no Windows.
            let value = value.trim_end();

            match key.trim() {
                "source_dir" => settings.source_dir = value.to_string(),
                "output_file" => settings.output_file = value.to_string(),
                _ => {} // chave desconhecida: ignorada (compat. futura)
            }
        }
    }

    settings
}

/// Grava o arquivo no formato `chave=valor`, recriando-o por completo.
fn save_to(path: &Path, settings: &Settings) {
    let content = format!(
        "# MSC Converter — caminhos usados na última execução \
         (gerado automaticamente; apague o arquivo para restaurar os padrões)\n\
         source_dir={}\n\
         output_file={}\n",
        settings.source_dir, settings.output_file
    );

    // Gravação direta: o arquivo tem poucos bytes.
    let _ = std::fs::write(path, content);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Arquivo temporário exclusivo por teste.
    fn temp_ini(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "mscc-settings-test-{}-{name}.ini",
            std::process::id()
        ))
    }

    #[test]
    fn arquivo_inexistente_retorna_padrao() {
        let path = temp_ini("missing");
        let _ = std::fs::remove_file(&path);

        assert_eq!(load_from(&path), Settings::default());
    }

    #[test]
    fn roundtrip_save_load() {
        let path = temp_ini("roundtrip");
        let settings = Settings {
            source_dir: r"C:\dados\msc".to_string(),
            output_file: r"D:\out\msc com espaço.duckdb".to_string(),
        };

        save_to(&path, &settings);
        assert_eq!(load_from(&path), settings);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn ignora_linhas_em_branco_comentarios_e_chaves_desconhecidas() {
        let path = temp_ini("comments");
        std::fs::write(
            &path,
            "# comentário\n\nsource_dir=/tmp/msc\noutra_chave=x\noutput_file=/tmp/o.duckdb\n",
        )
        .unwrap();

        assert_eq!(
            load_from(&path),
            Settings {
                source_dir: "/tmp/msc".to_string(),
                output_file: "/tmp/o.duckdb".to_string(),
            }
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn valor_pode_conter_sinal_de_igual() {
        let path = temp_ini("equals");
        std::fs::write(&path, "source_dir=C:\\a=b\n").unwrap();

        assert_eq!(load_from(&path).source_dir, r"C:\a=b");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn sobrescreve_valores_antigos() {
        let path = temp_ini("overwrite");

        save_to(
            &path,
            &Settings {
                source_dir: "antigo".to_string(),
                output_file: "antigo".to_string(),
            },
        );
        save_to(
            &path,
            &Settings {
                source_dir: "novo".to_string(),
                output_file: String::new(),
            },
        );

        assert_eq!(
            load_from(&path),
            Settings {
                source_dir: "novo".to_string(),
                output_file: String::new(),
            }
        );

        let _ = std::fs::remove_file(&path);
    }
}
