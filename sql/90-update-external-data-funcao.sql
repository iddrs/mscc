update msc
set
    nome_funcao = t.nome
    from msc_funcao t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.funcao;
