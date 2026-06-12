<?php

use \MscConverter\Support\Output;


// Prepara o ambiente

// Carrega as bibliotecas necessárias
require "vendor/autoload.php";


// Exibe mensagem inicial
Output::writenl('========================================================================');
Output::writenl('MSC Converter');
Output::writenl('Conversor ddo *.csv da MSC para DuckDB');
Output::writenl();
Output::writenl('Desenvolvido por Everton da Rosa <everton3x@gmail.com>');
Output::writenl('========================================================================');
Output::writenl();

// Processa os parâmetros de entrada
if($argc < 3) {
    Output::showHelp();
}

$source_dir = $argv[1] ?? null;
$output_file = $argv[2] ?? null;
if(is_null($source_dir) || is_null($output_file)) {
    Output::showHelp();
    exit;
}

if(!is_dir($source_dir)) {
    Output::println("$source_dir não é um diretório.");
    exit();
}

$source_dir = realpath($source_dir);
if(!file_exists($source_dir)) {
    Output::writenl("Diretório da MSC não encontrado: $source_dir");
    exit;
}
Output::writenl("Source dir: $source_dir");
Output::writenl("Output file: $output_file");

// Iniciando o processamento
$processor = new \MscConverter\Convert\Process($source_dir, $output_file);
$processor->run();