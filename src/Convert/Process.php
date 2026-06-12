<?php

namespace MscConverter\Convert;

use MscConverter\Support\Output;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;
use RecursiveRegexIterator;
use RegexIterator;
use Saturio\DuckDB\DuckDB;
use Saturio\DuckDB\DB\Configuration;

class Process
{
    private array $context = [];
    private ?DuckDB $db = null;

    public function __construct(
        public readonly string $sourceDir,
        public readonly string $outputFile
    )
    {

    }

    public function run(): void
    {
        $this->connectDb();
        foreach ($this->getSourceFiles() as $sourceFile) {
            $this->readContext($sourceFile);
            $this->executeQueries();
        }
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

    private function getSourceFiles(): array
    {
        $sourceFiles = [];
        $dirIterator = new RegexIterator(
            new RecursiveIteratorIterator(
                new RecursiveDirectoryIterator($this->sourceDir, RecursiveDirectoryIterator::SKIP_DOTS)), '/^.+\.csv$/i', RecursiveRegexIterator::GET_MATCH);
        foreach ($dirIterator as $item) {
                $sourceFiles[] = realpath($item[0]);
        }
        return $sourceFiles;
    }

    private function readContext(string $sourceFile): void
    {
        $fh = fopen($sourceFile, 'r');
        $header = fgetcsv($fh, null, ';', '"', '\\');
        $this->context = [
            '{{source_file}}' => $sourceFile,
            '{{code}}' => $header[0],
            '{{remessa}}' => (int) str_replace("-", '', $header[1]),
        ];
        Output::writenl("Código:\t\t{$this->context['{{code}}']}");
        Output::writenl("Remessa:\t{$this->context['{{remessa}}']}");
        fclose($fh);
    }
}