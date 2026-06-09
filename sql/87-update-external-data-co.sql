update msc
set
    nome_codigo_orcamentario = t.nome
    from msc_co t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.codigo_orcamentario;
