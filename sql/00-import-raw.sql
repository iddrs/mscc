create table if not exists msc_raw as
    select '' as chave, '{{remessa}}' as remessa, *
    from read_csv(
        '{{source_file}}',
        all_varchar = true,
        allow_quoted_nulls = true,
        decimal_separator = '.',
        delim = ';',
        header = true,
        skip = 1
    );