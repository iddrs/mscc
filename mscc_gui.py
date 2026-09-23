#!/usr/bin/env python3
"""GUI (tkinter) para o mscc — conversor de *.csv da MSC para DuckDB.

Uso:
    python3 mscc_gui.py [source_dir] [output_file]

Executa: php mscc.php source_dir output_file
e exibe a saída crua da ferramenta numa área de texto.
"""

import queue
import shutil
import subprocess
import sys
import threading
import tkinter as tk
import tkinter.font as tkfont
from pathlib import Path
from tkinter import filedialog, messagebox, scrolledtext, ttk

TOOL_NAME = "mscc"
SCRIPT_DIR = Path(__file__).resolve().parent
MSCC_PHP = SCRIPT_DIR / "mscc.php"

# Palavras-chave usadas para destacar linhas de erro na saída.
ERR_HINTS = (
    "erro", "error", "exception", "fatal", "traceback",
    "não é", "não encontrado", "not found", "failed", "invalid",
)


class MsccGui(tk.Tk):
    def __init__(self) -> None:
        super().__init__()
        self.title(f"{TOOL_NAME} — MSC Converter (DuckDB)")
        self.geometry("880x660")
        self.minsize(680, 500)

        self.queue: "queue.Queue[tuple[str, object]]" = queue.Queue()
        self.proc: subprocess.Popen | None = None
        self.cancel_requested = False

        self.var_source = tk.StringVar()
        self.var_output = tk.StringVar()
        self.status = tk.StringVar(value="Pronto.")

        self._build_ui()
        self._prefill_from_argv()
        self.after(100, self._poll_output)
        self.protocol("WM_DELETE_WINDOW", self._on_close)

    # ------------------------------------------------------------------ UI
    def _build_ui(self) -> None:
        pad = {"padx": 8, "pady": 4}

        # Cabeçalho
        header = ttk.Frame(self)
        header.pack(fill="x", **pad)
        ttk.Label(
            header,
            text="Conversor de *.csv da MSC para DuckDB",
            font=("TkDefaultFont", 12, "bold"),
        ).pack(anchor="w")
        ttk.Label(
            header,
            text="mscc source_dir output_file",
            font=("TkFixedFont",),
            foreground="#555555",
        ).pack(anchor="w")

        # Campos de entrada
        form = ttk.LabelFrame(self, text="Parâmetros")
        form.pack(fill="x", **pad)
        form.columnconfigure(1, weight=1)

        ttk.Label(form, text="Diretório da MSC (source_dir):").grid(
            row=0, column=0, sticky="w", padx=6, pady=4
        )
        ttk.Entry(form, textvariable=self.var_source).grid(
            row=0, column=1, sticky="ew", padx=6, pady=4
        )
        ttk.Button(form, text="Procurar…", command=self._browse_source).grid(
            row=0, column=2, padx=6, pady=4
        )

        ttk.Label(form, text="Arquivo DuckDB (output_file):").grid(
            row=1, column=0, sticky="w", padx=6, pady=4
        )
        entry_output = ttk.Entry(form, textvariable=self.var_output)
        entry_output.grid(row=1, column=1, sticky="ew", padx=6, pady=4)
        entry_output.bind("<Return>", lambda _e: self._run_conversion())
        ttk.Button(form, text="Procurar…", command=self._browse_output).grid(
            row=1, column=2, padx=6, pady=4
        )

        # Botões de controle
        controls = ttk.Frame(self)
        controls.pack(fill="x", **pad)
        self.btn_run = ttk.Button(
            controls, text="▶ Executar conversão", command=self._run_conversion
        )
        self.btn_run.pack(side="left", padx=(0, 6))
        self.btn_cancel = ttk.Button(
            controls, text="■ Cancelar", command=self._cancel, state="disabled"
        )
        self.btn_cancel.pack(side="left", padx=(0, 6))
        ttk.Button(controls, text="Limpar saída", command=self._clear_output).pack(
            side="right", padx=(6, 0)
        )
        ttk.Button(controls, text="Copiar saída", command=self._copy_output).pack(
            side="right", padx=(6, 0)
        )

        # Saída crua da ferramenta
        out_frame = ttk.LabelFrame(self, text="Saída")
        out_frame.pack(fill="both", expand=True, **pad)
        self.text = scrolledtext.ScrolledText(
            out_frame,
            wrap="none",
            state="disabled",
            font=("TkFixedFont",),
            background="#1e1e1e",
            foreground="#d4d4d4",
            insertbackground="#d4d4d4",
        )
        self.text.pack(fill="both", expand=True, padx=4, pady=4)
        self.text.tag_configure("err", foreground="#ff6b6b")
        mono_bold = tkfont.nametofont("TkFixedFont").copy()
        mono_bold.configure(weight="bold")
        self.text.tag_configure("cmd", foreground="#82aaff", font=mono_bold)

        # Barra de status
        ttk.Label(self, textvariable=self.status, relief="sunken", anchor="w").pack(
            fill="x", side="bottom"
        )

        self.bind("<Control-c>", lambda _e: self._copy_output())

    def _prefill_from_argv(self) -> None:
        if len(sys.argv) > 1:
            self.var_source.set(sys.argv[1])
        if len(sys.argv) > 2:
            self.var_output.set(sys.argv[2])

    # ------------------------------------------------------------ arquivos
    def _browse_source(self) -> None:
        folder = filedialog.askdirectory(
            title="Selecione o diretório com os *.csv da MSC"
        )
        if folder:
            self.var_source.set(folder)

    def _browse_output(self) -> None:
        fname = filedialog.asksaveasfilename(
            title="Selecione o arquivo DuckDB de saída",
            defaultextension=".duckdb",
            initialfile="msc.duckdb",
            filetypes=[
                ("Arquivo DuckDB", "*.duckdb"),
                ("Todos os arquivos", "*.*"),
            ],
        )
        if fname:
            self.var_output.set(fname)

    # ------------------------------------------------------------- execução
    def _run_conversion(self) -> None:
        src = self.var_source.get().strip()
        out = self.var_output.get().strip()

        if not src:
            messagebox.showerror("Erro", "Informe o diretório de origem (*.csv da MSC).")
            return
        if not Path(src).is_dir():
            messagebox.showerror("Erro", f"O diretório não existe:\n{src}")
            return
        if not out:
            messagebox.showerror("Erro", "Informe o caminho do arquivo DuckDB de saída.")
            return

        php = shutil.which("php")
        if not php:
            messagebox.showerror(
                "Erro", "Binário 'php' não encontrado no PATH.\nInstale o PHP 8.5+ e tente novamente."
            )
            return

        out_path = Path(out)
        out_dir = out_path.parent
        if str(out_dir) and not out_dir.exists():
            if not messagebox.askyesno(
                "Confirmação",
                f"O diretório de saída não existe:\n{out_dir}\n\nDeseja criá-lo?",
            ):
                return
            try:
                out_dir.mkdir(parents=True, exist_ok=True)
            except OSError as exc:
                messagebox.showerror("Erro", f"Não foi possível criar o diretório:\n{exc}")
                return

        cmd = [php, str(MSCC_PHP), src, out]
        self._append(f"$ {' '.join(cmd)}\n", "cmd")
        self._append("=" * 72 + "\n", None)

        try:
            self.proc = subprocess.Popen(
                cmd,
                cwd=str(SCRIPT_DIR),  # sql/*.sql e external_data.xlsx são relativos ao CWD
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,  # mescla para preservar a ordem da saída
                text=True,
                encoding="utf-8",
                errors="replace",
                bufsize=1,
            )
        except OSError as exc:
            self._append(f"Erro ao executar o PHP: {exc}\n", "err")
            messagebox.showerror("Erro", f"Não foi possível executar o PHP:\n{exc}")
            return

        self.cancel_requested = False
        self._set_running(True)
        self.status.set(f"Executando… (PID {self.proc.pid})")
        threading.Thread(target=self._read_stream, args=(self.proc,), daemon=True).start()

    def _read_stream(self, proc: subprocess.Popen) -> None:
        try:
            for line in proc.stdout:
                self.queue.put(("out", line))
        finally:
            rc = proc.wait()
            self.queue.put(("done", rc))

    def _cancel(self) -> None:
        if self.proc is None or self.proc.poll() is not None:
            return
        self.cancel_requested = True
        self.status.set("Cancelando…")
        try:
            self.proc.terminate()
        except OSError:
            pass

        def _kill_if_needed() -> None:
            try:
                self.proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                try:
                    self.proc.kill()
                except OSError:
                    pass

        threading.Thread(target=_kill_if_needed, daemon=True).start()

    def _set_running(self, running: bool) -> None:
        if running:
            self.btn_run.config(state="disabled")
            self.btn_cancel.config(state="normal")
        else:
            self.btn_run.config(state="normal")
            self.btn_cancel.config(state="disabled")

    # --------------------------------------------------------------- saída
    def _poll_output(self) -> None:
        try:
            while True:
                kind, payload = self.queue.get_nowait()
                if kind == "out":
                    self._append(str(payload), self._tag_for(str(payload)))
                elif kind == "done":
                    rc = payload
                    self._set_running(False)
                    if rc == 0:
                        self.status.set(f"Concluído (código {rc}).")
                    elif self.cancel_requested:
                        self.status.set("Cancelado.")
                    else:
                        self.status.set(f"Concluído com erro (código {rc}).")
        except queue.Empty:
            pass
        self.after(100, self._poll_output)

    def _append(self, text: str, tag: str | None) -> None:
        self.text.configure(state="normal")
        self.text.insert("end", text, tag)
        self.text.see("end")
        self.text.configure(state="disabled")

    @staticmethod
    def _tag_for(text: str) -> str | None:
        low = text.lower()
        if any(hint in low for hint in ERR_HINTS):
            return "err"
        return None

    def _clear_output(self) -> None:
        self.text.configure(state="normal")
        self.text.delete("1.0", "end")
        self.text.configure(state="disabled")

    def _copy_output(self) -> None:
        content = self.text.get("1.0", "end-1c")
        if content:
            self.clipboard_clear()
            self.clipboard_append(content)
            self.update_idletasks()
            self.status.set("Saída copiada para a área de transferência.")

    # --------------------------------------------------------------- sair
    def _on_close(self) -> None:
        if self.proc is not None and self.proc.poll() is None:
            if not messagebox.askyesno(
                "Sair", "A conversão ainda está em execução. Deseja cancelar e sair?"
            ):
                return
            try:
                self.proc.terminate()
            except OSError:
                pass
        self.destroy()


def main() -> None:
    app = MsccGui()
    app.mainloop()


if __name__ == "__main__":
    main()