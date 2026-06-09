update msc
set
    nome_fonte_recurso = t.nome
    from msc_fr t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.fonte_recurso;
