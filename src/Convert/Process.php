<?php

namespace MscConverter\Convert;

use MscConverter\Support\Output;
use Saturio\DuckDB\DuckDB;
use Saturio\DuckDB\DB\Configuration;

class Process
{
    private array $context = [];
    private ?DuckDB $db = null;

    public function __construct(
        public readonly string $sourceFile,
        public readonly string $outputFile
    )
    {

    }

    public function run(): void
    {
        $this->readContext();
        $this->connectDb();
        $this->executeQueries();
    }

    private function executeQueries(): void
    {
        $queries = glob('./sql/*.sql');
        foreach ($queries as $qfile) {
            Output::writenl($qfile);
            $query = file_get_contents($qfile);
            $query = str_replace(array_keys($this->context), array_values($this->context), $query);
            $this->db->query($query);
        }
    }

    private function connectDb(): void
    {
        $duckConfig = new Configuration();
        $duckConfig->set('access_mode', 'READ_WRITE');
        $this->db = DuckDB::create($this->outputFile, config: $duckConfig);
    }
    private function readContext(): void
    {
        $fh = fopen($this->sourceFile, 'r');
        $header = fgetcsv($fh, null, ';', '"', '\\');
        $this->context = [
            '{{source_file}}' => $this->sourceFile,
            '{{code}}' => $header[0],
            '{{remessa}}' => (int) str_replace("-", '', $header[1]),
        ];
        Output::writenl("Código:\t\t{$this->context['{{code}}']}");
        Output::writenl("Remessa:\t{$this->context['{{remessa}}']}");
        fclose($fh);
    }
}