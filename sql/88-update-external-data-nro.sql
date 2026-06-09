update msc
set
    nome_natureza_receita = t.nome
    from msc_nro t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod like msc.natureza_receita;
