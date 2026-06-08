update msc
set
    natureza_receita =
        substring(natureza_receita, 1, 1) || '.' ||
        substring(natureza_receita, 2, 1) || '.' ||
        substring(natureza_receita, 3, 1) || '.' ||
        substring(natureza_receita, 4, 1) || '.' ||
        substring(natureza_receita, 5, 2) || '.' ||
        substring(natureza_receita, 7, 1) || '.' ||
        substring(natureza_receita, 8, 1),
    natureza_despesa =
        substring(natureza_despesa, 1, 1) || '.' ||
        substring(natureza_despesa, 2, 1) || '.' ||
        substring(natureza_despesa, 3, 2) || '.' ||
        substring(natureza_despesa, 5, 2) || '.' ||
        substring(natureza_despesa, 7, 2)
where remessa = {{remessa}};