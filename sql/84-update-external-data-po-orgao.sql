update msc
set
    nome_orgao = t.nm_orgao
from msc_po t
where
    msc.remessa = {{remessa}}
    and t.exercicio = cast(substring(cast(remessa as varchar(6)), 1, 4) as uinteger)
    and t.cd_orgao = msc.orgao;
