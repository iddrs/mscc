<?php return array(
    'root' => array(
        'name' => 'contabilidade/mscc',
        'pretty_version' => '1.0.0+no-version-set',
        'version' => '1.0.0.0',
        'reference' => null,
        'type' => 'tool',
        'install_path' => __DIR__ . '/../../',
        'aliases' => array(),
        'dev' => true,
    ),
    'versions' => array(
        'contabilidade/mscc' => array(
            'pretty_version' => '1.0.0+no-version-set',
            'version' => '1.0.0.0',
            'reference' => null,
            'type' => 'tool',
            'install_path' => __DIR__ . '/../../',
            'aliases' => array(),
            'dev_requirement' => false,
        ),
        'satur.io/duckdb' => array(
            'pretty_version' => 'dev-main',
            'version' => 'dev-main',
            'reference' => 'd09caa5a0ea630683b707d739fc5d055bd9427e9',
            'type' => 'library',
            'install_path' => __DIR__ . '/../satur.io/duckdb',
            'aliases' => array(
                0 => '9999999-dev',
            ),
            'dev_requirement' => false,
        ),
    ),
);
