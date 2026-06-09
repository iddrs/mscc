update msc
set
    nome_subfuncao = t.nome
    from msc_subfuncao t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.subfuncao;

