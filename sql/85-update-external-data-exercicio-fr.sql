update msc
set
    nome_exercicio_fonte_recurso = t.nome
    from msc_exercicio_fr t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.exercicio_fonte_recurso;
