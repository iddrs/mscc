update msc
set
    nome_conta_contabil = t.nome
from pcasp t
where
    msc.remessa = {{remessa}}
    and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
    and t.cod like msc.conta_contabil;

update msc
set
    nome_poder = t.nm_poder
from po t
where
    msc.remessa = {{remessa}}
    and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
    and t.cd_poder = msc.poder;

update msc
set
    nome_orgao = t.nm_orgao
from po t
where
    msc.remessa = {{remessa}}
    and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
    and t.cd_orgao = msc.orgao;

update msc
set
    nome_exercicio_fonte_recurso = t.nome
    from exercicio_fr t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.exercicio_fonte_recurso;

update msc
set
    nome_fonte_recurso = t.nome
    from fr t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.fonte_recurso;

update msc
set
    nome_codigo_orcamentario = t.nome
    from co t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.codigo_orcamentario;

update msc
set
    nome_natureza_receita = t.nome
    from nro t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod like msc.natureza_receita;

update msc
set
    nome_natureza_despesa = t.nome
    from ndo t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod like msc.natureza_despesa;

update msc
set
    nome_funcao = t.nome
    from funcao t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.funcao;

update msc
set
    nome_subfuncao = t.nome
    from subfuncao t
where
    msc.remessa = {{remessa}}
  and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
  and t.cod = msc.subfuncao;

