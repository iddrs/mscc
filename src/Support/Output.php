<?php

namespace MscConverter\Support;

class Output
{
    static public function writenl(?string $message = null): void
    {
        if(is_null($message)){
            echo PHP_EOL;
            return;
        }
        echo $message, PHP_EOL;
    }

    static public function showHelp(): void
    {
        self::writenl('mscc source_file output_file');
        self::writenl();
        self::writenl('Argumentos:');
        self::writenl("\tsource_file\tCaminho para o arquivo *.csv da MSC.");
        self::writenl("\toutput_file\tCaminho para o arquivo DuckDB.");
    }
}