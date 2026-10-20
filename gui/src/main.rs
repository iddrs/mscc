//! MSC Converter — interface gráfica (Iced) para o conversor CSV da MSC → DuckDB.
//!
//! A GUI executa o mesmo comando da linha de comando:
//!
//! ```text
//! php mscc.php <source_dir> <output_file>
//! ```
//!
//! A saída *raw* do script PHP (stdout + stderr) é exibida em tempo real numa
//! área de texto com scrollbar.

use std::path::{Path, PathBuf};

mod settings;

use iced::widget::{
    button, column, container, row, scrollable, space, text, text_input, Column,
};
use iced::{
    window, Alignment, Background, Border, Color, Element, Font, Length, Padding,
    Size, Task, Theme,
};
use iced::futures::SinkExt;

/// Nome do programa (exibido no cabeçalho e na janela).
const APP_NAME: &str = "MSC Converter";
/// Função do programa (exibida no cabeçalho).
const APP_DESCRIPTION: &str = "Conversor dos *.csv da MSC para DuckDB";
/// Uso do comando (exibido no cabeçalho).
const USAGE: &str = "mscc source_dir output_file";
/// Informações do desenvolvedor (exibidas na barra de status).
const DEVELOPER: &str = "Desenvolvido por Everton da Rosa <everton3x@gmail.com>";

/// Máximo de linhas mantidas na área de saída.
const MAX_OUTPUT_LINES: usize = 5_000;

/// Palavras-chave que identificam linhas de erro na saída do script.
const ERROR_HINTS: [&str; 10] = [
    "erro",
    "error",
    "exception",
    "fatal",
    "traceback",
    "não é",
    "não encontrado",
    "not found",
    "failed",
    "invalid",
];

// ---------------------------------------------------------------------------
// Cores do visual dark
// ---------------------------------------------------------------------------

const BG: Color = Color::from_rgb(0.078, 0.090, 0.118); // fundo da janela
const PANEL_BG: Color = Color::from_rgb(0.110, 0.129, 0.161); // painéis
const PANEL_BORDER: Color = Color::from_rgb(0.204, 0.231, 0.278); // bordas
const INPUT_BG: Color = Color::from_rgb(0.075, 0.086, 0.110); // campos / log
const TEXT: Color = Color::from_rgb(0.878, 0.898, 0.918); // texto normal
const MUTED: Color = Color::from_rgb(0.545, 0.584, 0.631); // texto secundário
const ACCENT: Color = Color::from_rgb(0.263, 0.518, 0.878); // destaque
const ACCENT_HOVER: Color = Color::from_rgb(0.322, 0.584, 0.929);
const ACCENT_PRESSED: Color = Color::from_rgb(0.208, 0.451, 0.776);
const BTN_BG: Color = Color::from_rgb(0.173, 0.200, 0.247); // botões neutros
const BTN_BG_HOVER: Color = Color::from_rgb(0.220, 0.255, 0.310);
const BTN_BG_PRESSED: Color = Color::from_rgb(0.137, 0.161, 0.200);
const LOG_TEXT: Color = Color::from_rgb(0.820, 0.855, 0.890); // saída normal
const LOG_CMD: Color = Color::from_rgb(0.545, 0.741, 1.000); // comando executado
const LOG_ERROR: Color = Color::from_rgb(1.000, 0.420, 0.420); // erros
const DANGER: Color = Color::from_rgb(0.850, 0.306, 0.306); // botão cancelar

/// Cria o processo PHP sem abrir um console extra no Windows.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// ---------------------------------------------------------------------------
// Estado
// ---------------------------------------------------------------------------

/// Tipo de uma linha exibida na área de saída.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineKind {
    Normal,
    Command,
    Error,
}

impl LineKind {
    fn color(self) -> Color {
        match self {
            LineKind::Normal => LOG_TEXT,
            LineKind::Command => LOG_CMD,
            LineKind::Error => LOG_ERROR,
        }
    }
}

/// Uma linha da saída *raw* do script PHP.
struct OutputLine {
    content: String,
    kind: LineKind,
}

/// Estado da aplicação.
struct App {
    /// Corresponde ao argumento `source_dir` do `mscc.php`.
    source_dir: String,
    /// Corresponde ao argumento `output_file` do `mscc.php`.
    output_file: String,
    /// Linhas da saída do script.
    output: Vec<OutputLine>,
    /// Mensagem exibida junto aos botões.
    status: String,
    /// `true` quando a última mensagem de status indica erro.
    status_error: bool,
    /// `true` enquanto o processo PHP está em execução.
    running: bool,
    /// Handle usado para cancelar (abortar) a execução em curso.
    run_handle: Option<iced::task::Handle>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let mut args = std::env::args().skip(1);

        // Argumentos da linha de comando têm prioridade sobre os valores
        // salvos na execução anterior (settings::FILE_NAME).
        let saved = settings::load();

        let source_dir = args
            .next()
            .filter(|value| !value.is_empty())
            .unwrap_or(saved.source_dir);
        let output_file = args
            .next()
            .filter(|value| !value.is_empty())
            .unwrap_or(saved.output_file);

        let app = App {
            source_dir,
            output_file,
            output: Vec::new(),
            status: "Pronto. Selecione os caminhos e clique em Executar.".to_string(),
            status_error: false,
            running: false,
            run_handle: None,
        };

        (app, Task::none())
    }

    /// Persiste os caminhos informados para a próxima inicialização.
    fn persist_settings(&self) {
        settings::save(&settings::Settings {
            source_dir: self.source_dir.clone(),
            output_file: self.output_file.clone(),
        });
    }

    /// Acrescenta uma linha à área de saída, limitando o total armazenado.
    fn push_line(&mut self, content: impl Into<String>, kind: LineKind) {
        let content = content.into();

        // Linhas vazias precisam de altura; um espaço preserva o espaçamento.
        let content = if content.is_empty() {
            " ".to_string()
        } else {
            content
        };

        self.output.push(OutputLine { content, kind });

        if self.output.len() > MAX_OUTPUT_LINES + 512 {
            let excess = self.output.len() - MAX_OUTPUT_LINES;
            self.output.drain(..excess);
        }
    }

    /// Define o status como erro (e registra na área de saída).
    fn fail(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.status = message.clone();
        self.status_error = true;
        self.push_line(message, LineKind::Error);
    }

    /// Valida os parâmetros e inicia a execução do `php mscc.php`.
    fn start_run(&mut self, script: PathBuf, source: String, output: String) -> Task<Message> {
        let command = format!("php {} \"{}\" \"{}\"", script.display(), source, output);

        self.push_line(format!("$ {command}"), LineKind::Command);
        self.push_line("=".repeat(72), LineKind::Normal);

        self.running = true;
        self.status_error = false;
        self.status = format!("Executando… ({command})");

        // `sql/*.sql` e `external_data.xlsx` são relativos ao diretório do script.
        let workdir = script.parent().map(Path::to_path_buf);

        let stream = iced::stream::channel(256, async move |mut sender| {
            let mut command = tokio::process::Command::new("php");
            command
                .arg(&script)
                .arg(&source)
                .arg(&output)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                // Cancelar/fechar a GUI encerra o processo filho.
                .kill_on_drop(true);

            if let Some(workdir) = &workdir {
                command.current_dir(workdir);
            }

            #[cfg(windows)]
            command.creation_flags(CREATE_NO_WINDOW);

            let mut child = match command.spawn() {
                Ok(child) => child,
                Err(error) => {
                    let _ = sender.send(Message::SpawnFailed(error.to_string())).await;
                    return;
                }
            };

            use tokio::io::AsyncBufReadExt;

            let mut stdout = tokio::io::BufReader::new(
                child.stdout.take().expect("stdout do processo"),
            )
            .lines();
            let mut stderr = tokio::io::BufReader::new(
                child.stderr.take().expect("stderr do processo"),
            )
            .lines();

            let mut stdout_open = true;
            let mut stderr_open = true;

            while stdout_open || stderr_open {
                tokio::select! {
                    line = stdout.next_line(), if stdout_open => match line {
                        Ok(Some(text)) => {
                            let _ = sender.send(Message::OutputLine(text)).await;
                        }
                        Ok(None) => stdout_open = false,
                        Err(error) => {
                            let _ = sender
                                .send(Message::OutputLine(format!("Erro ao ler a saída: {error}")))
                                .await;
                            stdout_open = false;
                        }
                    },
                    line = stderr.next_line(), if stderr_open => match line {
                        Ok(Some(text)) => {
                            let _ = sender.send(Message::OutputLine(text)).await;
                        }
                        Ok(None) => stderr_open = false,
                        Err(error) => {
                            let _ = sender
                                .send(Message::OutputLine(format!("Erro ao ler o erro: {error}")))
                                .await;
                            stderr_open = false;
                        }
                    },
                }
            }

            match child.wait().await {
                Ok(status) => {
                    let _ = sender
                        .send(Message::Finished(status.code().unwrap_or(-1)))
                        .await;
                }
                Err(error) => {
                    let _ = sender
                        .send(Message::SpawnFailed(format!(
                            "Erro ao finalizar o PHP: {error}"
                        )))
                        .await;
                }
            }
        });

        let (task, handle) = Task::stream(stream).abortable();
        self.run_handle = Some(handle);

        task
    }
}

/// Procura o `mscc.php` ao lado do executável ou no diretório atual.
fn find_mscc_script() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        candidates.push(dir.join("mscc.php"));
    }

    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("mscc.php"));
    }

    candidates.into_iter().find(|path| path.is_file())
}

/// Classifica uma linha da saída (para colorir erros em vermelho).
fn classify(line: &str) -> LineKind {
    let lowered = line.to_lowercase();

    if ERROR_HINTS.iter().any(|hint| lowered.contains(hint)) {
        LineKind::Error
    } else {
        LineKind::Normal
    }
}

// ---------------------------------------------------------------------------
// Mensagens
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    SourceChanged(String),
    OutputChanged(String),
    PickSource,
    PickOutput,
    SourcePicked(Option<PathBuf>),
    OutputPicked(Option<PathBuf>),
    Run,
    Cancel,
    ClearOutput,
    OutputLine(String),
    Finished(i32),
    SpawnFailed(String),
}

// ---------------------------------------------------------------------------
// Lógica
// ---------------------------------------------------------------------------

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::SourceChanged(value) => {
            app.source_dir = value;
            app.persist_settings();
            Task::none()
        }
        Message::OutputChanged(value) => {
            app.output_file = value;
            app.persist_settings();
            Task::none()
        }
        Message::PickSource => Task::perform(
            async {
                rfd::AsyncFileDialog::new()
                    .set_title("Selecione o diretório com os *.csv da MSC")
                    .pick_folder()
                    .await
                    .map(|handle| handle.path().to_path_buf())
            },
            Message::SourcePicked,
        ),
        Message::PickOutput => {
            let mut dialog = rfd::AsyncFileDialog::new()
                .set_title("Selecione o arquivo DuckDB de saída")
                .set_file_name("msc.duckdb")
                .add_filter("Arquivo DuckDB", &["duckdb"])
                .add_filter("Todos os arquivos", &["*"]);

            // Abre o diálogo no diretório do arquivo atual, quando houver.
            if let Some(dir) = Path::new(&app.output_file).parent()
                && dir.is_dir()
            {
                dialog = dialog.set_directory(dir);
            }

            Task::perform(
                async move {
                    dialog
                        .pick_file()
                        .await
                        .map(|handle| handle.path().to_path_buf())
                },
                Message::OutputPicked,
            )
        }
        Message::SourcePicked(Some(path)) => {
            app.source_dir = path.display().to_string();
            app.persist_settings();
            Task::none()
        }
        Message::OutputPicked(Some(path)) => {
            app.output_file = path.display().to_string();
            app.persist_settings();
            Task::none()
        }
        Message::SourcePicked(None) | Message::OutputPicked(None) => Task::none(),
        Message::Run => {
            if app.running {
                return Task::none();
            }

            let source = app.source_dir.trim().to_string();
            let output = app.output_file.trim().to_string();

            if source.is_empty() {
                app.fail("Informe o diretório de origem (*.csv da MSC).");
                return Task::none();
            }

            if !Path::new(&source).is_dir() {
                app.fail(format!("O diretório não existe: {source}"));
                return Task::none();
            }

            if output.is_empty() {
                app.fail("Informe o caminho do arquivo DuckDB de saída.");
                return Task::none();
            }

            // Cria o diretório de saída, se necessário.
            if let Some(parent) = Path::new(&output).parent()
                && !parent.as_os_str().is_empty()
                && !parent.is_dir()
            {
                if let Err(error) = std::fs::create_dir_all(parent) {
                    app.fail(format!(
                        "Não foi possível criar o diretório de saída {}: {error}",
                        parent.display()
                    ));
                    return Task::none();
                }

                app.push_line(
                    format!("Diretório de saída criado: {}", parent.display()),
                    LineKind::Normal,
                );
            }

            let Some(script) = find_mscc_script() else {
                app.fail(
                    "mscc.php não encontrado ao lado do executável nem no diretório atual.",
                );
                return Task::none();
            };

            app.start_run(script, source, output)
        }
        Message::Cancel => {
            if let Some(handle) = app.run_handle.take() {
                handle.abort();
            }

            if app.running {
                app.running = false;
                app.push_line("Conversão cancelada pelo usuário.", LineKind::Error);
                app.status = "Cancelado pelo usuário.".to_string();
                app.status_error = true;
            }

            Task::none()
        }
        Message::ClearOutput => {
            app.output.clear();
            Task::none()
        }
        Message::OutputLine(line) => {
            let kind = classify(&line);
            app.push_line(line, kind);
            Task::none()
        }
        Message::Finished(code) => {
            app.running = false;
            app.run_handle = None;
            app.push_line("=".repeat(72), LineKind::Normal);

            if code == 0 {
                app.status = "Concluído com sucesso (código 0).".to_string();
                app.status_error = false;
            } else {
                app.push_line(
                    format!("O processo terminou com o código {code}."),
                    LineKind::Error,
                );
                app.status = format!("Concluído com erro (código {code}).");
                app.status_error = true;
            }

            Task::none()
        }
        Message::SpawnFailed(error) => {
            app.running = false;
            app.run_handle = None;
            app.fail(format!("Erro ao executar o PHP: {error}"));
            Task::none()
        }
    }
}

// ---------------------------------------------------------------------------
// Estilos
// ---------------------------------------------------------------------------

fn panel_style(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(PANEL_BG)),
        border: Border {
            color: PANEL_BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(TEXT),
        ..Default::default()
    }
}

fn output_panel_style(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(INPUT_BG)),
        border: Border {
            color: PANEL_BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        text_color: Some(LOG_TEXT),
        ..Default::default()
    }
}

fn status_bar_style(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(PANEL_BG)),
        border: Border {
            color: PANEL_BORDER,
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(MUTED),
        ..Default::default()
    }
}

fn input_style(_: &Theme, status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Active | text_input::Status::Focused { .. } => ACCENT,
        _ => PANEL_BORDER,
    };

    text_input::Style {
        background: Background::Color(INPUT_BG),
        icon: MUTED,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: 6.0.into(),
        },
        value: TEXT,
        placeholder: MUTED,
        selection: Color {
            a: 0.35,
            ..ACCENT
        },
    }
}

fn primary_button(_: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Active => ACCENT,
        button::Status::Hovered => ACCENT_HOVER,
        button::Status::Pressed => ACCENT_PRESSED,
        button::Status::Disabled => Color { a: 0.4, ..ACCENT },
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

fn secondary_button(_: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Active => BTN_BG,
        button::Status::Hovered => BTN_BG_HOVER,
        button::Status::Pressed => BTN_BG_PRESSED,
        button::Status::Disabled => Color {
            a: 0.35,
            ..BTN_BG
        },
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: TEXT,
        border: Border {
            color: PANEL_BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

fn danger_button(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = secondary_button(theme, status);
    style.text_color = match status {
        button::Status::Disabled => MUTED,
        _ => DANGER,
    };
    style
}

fn text_color(color: Color) -> impl Fn(&Theme) -> text::Style {
    move |_| text::Style { color: Some(color) }
}

// ---------------------------------------------------------------------------
// Visão
// ---------------------------------------------------------------------------

fn field_row<'a>(
    label: &'static str,
    placeholder: &'a str,
    value: &'a str,
    on_change: fn(String) -> Message,
    on_browse: Message,
    disabled: bool,
) -> Element<'a, Message> {
    row![
        text(label).width(170.0).size(14),
        text_input(placeholder, value)
            .on_input(on_change)
            .on_submit(Message::Run)
            .padding(10)
            .width(Length::Fill)
            .style(input_style),
        button(text("Procurar…"))
            .on_press_maybe((!disabled).then_some(on_browse))
            .padding(10)
            .style(secondary_button),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // Cabeçalho: nome e função do programa.
    let header = container(
        column![
            text(APP_NAME).size(26),
            text(APP_DESCRIPTION).size(15).style(text_color(MUTED)),
            text(USAGE)
                .font(Font::MONOSPACE)
                .size(12)
                .style(text_color(MUTED)),
        ]
        .spacing(6),
    )
    .style(panel_style)
    .padding(Padding::new(20.0));

    // Campos de entrada.
    let form = container(
        column![
            field_row(
                "Diretório da MSC",
                "Caminho do diretório com os *.csv da MSC",
                &app.source_dir,
                Message::SourceChanged,
                Message::PickSource,
                app.running,
            ),
            field_row(
                "Arquivo DuckDB",
                "Caminho do arquivo .duckdb de saída (pode ainda não existir)",
                &app.output_file,
                Message::OutputChanged,
                Message::PickOutput,
                app.running,
            ),
        ]
        .spacing(14),
    )
    .style(panel_style)
    .padding(Padding::new(20.0));

    // Botões de controle + situação atual.
    let status_color = if app.status_error {
        LOG_ERROR
    } else if app.running {
        ACCENT
    } else {
        MUTED
    };

    let controls = row![
        button(text("Executar"))
            .on_press_maybe((!app.running).then_some(Message::Run))
            .padding([10u16, 20u16])
            .style(primary_button),
        button(text("Cancelar"))
            .on_press_maybe(app.running.then_some(Message::Cancel))
            .padding([10u16, 20u16])
            .style(danger_button),
        button(text("Limpar saída"))
            .on_press(Message::ClearOutput)
            .padding([10u16, 20u16])
            .style(secondary_button),
        space().width(Length::Fill),
        text(&app.status)
            .size(13)
            .style(text_color(status_color)),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    // Saída raw do script PHP.
    let mut log = Column::new().spacing(2).width(Length::Fill);

    if app.output.is_empty() {
        log = log.push(
            text("A saída do `mscc.php` será exibida aqui.")
                .size(13)
                .style(text_color(MUTED)),
        );
    } else {
        for line in &app.output {
            log = log.push(
                text(line.content.as_str())
                    .font(Font::MONOSPACE)
                    .size(13)
                    .style(text_color(line.kind.color())),
            );
        }
    }

    let mut log_view = scrollable(log)
        .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::new(),
            horizontal: scrollable::Scrollbar::new(),
        })
        .height(Length::Fill);

    // Durante a execução, a visualização acompanha o final da saída.
    if app.running {
        log_view = log_view.anchor_bottom();
    }

    let output = container(log_view)
        .style(output_panel_style)
        .padding(Padding::new(12.0))
        .height(Length::Fill);

    // Barra de status com informações do desenvolvedor.
    let status_bar = container(
        row![
            text(APP_NAME).size(12).style(text_color(MUTED)),
            space().width(Length::Fill),
            text(DEVELOPER).size(12).style(text_color(MUTED)),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(status_bar_style)
    .padding(Padding::new(10.0));

    container(
        column![header, form, controls, output, status_bar]
            .spacing(14)
            .padding(Padding::new(16.0))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Ponto de entrada
// ---------------------------------------------------------------------------

fn main() -> iced::Result {
    iced::application(App::new, update, view)
        .title(|_: &App| format!("{APP_NAME} — {APP_DESCRIPTION}"))
        .theme(|_: &App| Theme::Dark)
        .style(|_: &App, _: &Theme| iced::theme::Style {
            background_color: BG,
            text_color: TEXT,
        })
        .window(window::Settings {
            size: Size::new(1000.0, 700.0),
            min_size: Some(Size::new(760.0, 560.0)),
            ..Default::default()
        })
        .centered()
        .run()
}
