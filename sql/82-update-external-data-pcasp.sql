update msc
set
    nome_conta_contabil = t.nome
from msc_pcasp t
where
    msc.remessa = {{remessa}}
    and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
    and t.cod like msc.conta_contabil;
