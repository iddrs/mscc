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

$source_file = $argv[1] ?? null;
$output_file = $argv[2] ?? null;
if(is_null($source_file) || is_null($output_file)) {
    Output::showHelp();
}

$source_file = realpath($source_file);
if(!file_exists($source_file)) {
    Output::writenl("Arquivo da MSC não encontrado: $source_file");
    exit;
}
Output::writenl("Source file: $source_file");
Output::writenl("Output file: $output_file");

// Iniciando o processamento
$processor = new \MscConverter\Convert\Process($source_file, $output_file);
$processor->run();