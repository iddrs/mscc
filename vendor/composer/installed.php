<?php return array(
    'root' => array(
        'name' => 'contabilidade/mscc',
        'pretty_version' => 'dev-main',
        'version' => 'dev-main',
        'reference' => '61bc5d82a254a86460f327872c16936cd94c0ea8',
        'type' => 'tool',
        'install_path' => __DIR__ . '/../../',
        'aliases' => array(),
        'dev' => true,
    ),
    'versions' => array(
        'contabilidade/mscc' => array(
            'pretty_version' => 'dev-main',
            'version' => 'dev-main',
            'reference' => '61bc5d82a254a86460f327872c16936cd94c0ea8',
            'type' => 'tool',
            'install_path' => __DIR__ . '/../../',
            'aliases' => array(),
            'dev_requirement' => false,
        ),
        'satur.io/duckdb' => array(
            'pretty_version' => 'dev-main',
            'version' => 'dev-main',
            'reference' => '9e30b059d5b86fc1e8025d448e20bdeeea21a0e3',
            'type' => 'library',
            'install_path' => __DIR__ . '/../satur.io/duckdb',
            'aliases' => array(
                0 => '9999999-dev',
            ),
            'dev_requirement' => false,
        ),
    ),
);
