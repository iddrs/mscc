delete from msc where remessa = {{remessa}};

insert into msc
(
    chave,
    remessa,
    conta_contabil,
    poder,
    orgao,
    indicador_superavit,
    divida_consolidada,
    exercicio_fonte_recurso,
    fonte_recurso,
    codigo_orcamentario,
    natureza_receita,
    natureza_despesa,
    funcao,
    subfuncao,
    ano_inscricao_restos_pagar
)
select distinct
    chave,
    remessa,
    (
        substring(ContaContabil, 1, 1) || '.' ||
        substring(ContaContabil, 2, 1) || '.' ||
        substring(ContaContabil, 3, 1) || '.' ||
        substring(ContaContabil, 4, 1) || '.' ||
        substring(ContaContabil, 5, 1) || '.' ||
        substring(ContaContabil, 6, 2) || '.' ||
        substring(ContaContabil, 8, 2)
    ) as conta_contabil,
    case
        when TipoInformacaoComplementar1 like 'PO' then cast(substring(InformacaoComplementar1, 1, 2) as uinteger)
        when TipoInformacaoComplementar2 like 'PO' then cast(substring(InformacaoComplementar2, 1, 2) as uinteger)
        when TipoInformacaoComplementar3 like 'PO' then cast(substring(InformacaoComplementar3, 1, 2) as uinteger)
        when TipoInformacaoComplementar4 like 'PO' then cast(substring(InformacaoComplementar4, 1, 2) as uinteger)
        when TipoInformacaoComplementar5 like 'PO' then cast(substring(InformacaoComplementar5, 1, 2) as uinteger)
        when TipoInformacaoComplementar6 like 'PO' then cast(substring(InformacaoComplementar6, 1, 2) as uinteger)
    end as poder,
    case
        when TipoInformacaoComplementar1 like 'PO' then cast(substring(InformacaoComplementar1, 3) as uinteger)
        when TipoInformacaoComplementar2 like 'PO' then cast(substring(InformacaoComplementar2, 3) as uinteger)
        when TipoInformacaoComplementar3 like 'PO' then cast(substring(InformacaoComplementar3, 3) as uinteger)
        when TipoInformacaoComplementar4 like 'PO' then cast(substring(InformacaoComplementar4, 3) as uinteger)
        when TipoInformacaoComplementar5 like 'PO' then cast(substring(InformacaoComplementar5, 3) as uinteger)
        when TipoInformacaoComplementar6 like 'PO' then cast(substring(InformacaoComplementar6, 3) as uinteger)
    end as orgao,
    case
        when TipoInformacaoComplementar1 like 'FP' then case InformacaoComplementar1 when '1' then 'F' when '2' then 'P' else null end
        when TipoInformacaoComplementar2 like 'FP' then case InformacaoComplementar2 when '1' then 'F' when '2' then 'P' else null end
        when TipoInformacaoComplementar3 like 'FP' then case InformacaoComplementar3 when '1' then 'F' when '2' then 'P' else null end
        when TipoInformacaoComplementar4 like 'FP' then case InformacaoComplementar4 when '1' then 'F' when '2' then 'P' else null end
        when TipoInformacaoComplementar5 like 'FP' then case InformacaoComplementar5 when '1' then 'F' when '2' then 'P' else null end
        when TipoInformacaoComplementar6 like 'FP' then case InformacaoComplementar6 when '1' then 'F' when '2' then 'P' else null end
    end as indicador_superavit,
    case
        when TipoInformacaoComplementar1 like 'DC' then (case InformacaoComplementar1 when '0' then 'S' else 'N' end)
        when TipoInformacaoComplementar2 like 'DC' then (case InformacaoComplementar2 when '0' then 'S' else 'N' end)
        when TipoInformacaoComplementar3 like 'DC' then (case InformacaoComplementar3 when '0' then 'S' else 'N' end)
        when TipoInformacaoComplementar4 like 'DC' then (case InformacaoComplementar4 when '0' then 'S' else 'N' end)
        when TipoInformacaoComplementar5 like 'DC' then (case InformacaoComplementar5 when '0' then 'S' else 'N' end)
        when TipoInformacaoComplementar6 like 'DC' then (case InformacaoComplementar6 when '0' then 'S' else 'N' end)
    end as divida_consolidada,
    case
        when TipoInformacaoComplementar1 like 'FR' then cast(substring(InformacaoComplementar1, 1, 1) as utinyint)
        when TipoInformacaoComplementar2 like 'FR' then cast(substring(InformacaoComplementar2, 1, 1) as utinyint)
        when TipoInformacaoComplementar3 like 'FR' then cast(substring(InformacaoComplementar3, 1, 1) as utinyint)
        when TipoInformacaoComplementar4 like 'FR' then cast(substring(InformacaoComplementar4, 1, 1) as utinyint)
        when TipoInformacaoComplementar5 like 'FR' then cast(substring(InformacaoComplementar5, 1, 1) as utinyint)
        when TipoInformacaoComplementar6 like 'FR' then cast(substring(InformacaoComplementar6, 1, 1) as utinyint)
    end as exercicio_fonte_recurso,
    case
        when TipoInformacaoComplementar1 like 'FR' then cast(substring(InformacaoComplementar1, 2) as uinteger)
        when TipoInformacaoComplementar2 like 'FR' then cast(substring(InformacaoComplementar2, 2) as uinteger)
        when TipoInformacaoComplementar3 like 'FR' then cast(substring(InformacaoComplementar3, 2) as uinteger)
        when TipoInformacaoComplementar4 like 'FR' then cast(substring(InformacaoComplementar4, 2) as uinteger)
        when TipoInformacaoComplementar5 like 'FR' then cast(substring(InformacaoComplementar5, 2) as uinteger)
        when TipoInformacaoComplementar6 like 'FR' then cast(substring(InformacaoComplementar6, 2) as uinteger)
    end as fonte_recurso,
    case
        when TipoInformacaoComplementar1 like 'CO' then cast(InformacaoComplementar1 as uinteger)
        when TipoInformacaoComplementar2 like 'CO' then cast(InformacaoComplementar2 as uinteger)
        when TipoInformacaoComplementar3 like 'CO' then cast(InformacaoComplementar3 as uinteger)
        when TipoInformacaoComplementar4 like 'CO' then cast(InformacaoComplementar4 as uinteger)
        when TipoInformacaoComplementar5 like 'CO' then cast(InformacaoComplementar5 as uinteger)
        when TipoInformacaoComplementar6 like 'CO' then cast(InformacaoComplementar6 as uinteger)
    end as codigo_orcamentario,
    case
        when TipoInformacaoComplementar1 like 'NR' then InformacaoComplementar1
        when TipoInformacaoComplementar2 like 'NR' then InformacaoComplementar2
        when TipoInformacaoComplementar3 like 'NR' then InformacaoComplementar3
        when TipoInformacaoComplementar4 like 'NR' then InformacaoComplementar4
        when TipoInformacaoComplementar5 like 'NR' then InformacaoComplementar5
        when TipoInformacaoComplementar6 like 'NR' then InformacaoComplementar6
    end as natureza_receita,
    case
        when TipoInformacaoComplementar1 like 'ND' then InformacaoComplementar1
        when TipoInformacaoComplementar2 like 'ND' then InformacaoComplementar2
        when TipoInformacaoComplementar3 like 'ND' then InformacaoComplementar3
        when TipoInformacaoComplementar4 like 'ND' then InformacaoComplementar4
        when TipoInformacaoComplementar5 like 'ND' then InformacaoComplementar5
        when TipoInformacaoComplementar6 like 'ND' then InformacaoComplementar6
    end as natureza_despesa,
    case
        when TipoInformacaoComplementar1 like 'FS' then cast(substring(InformacaoComplementar1, 1, 2) as uinteger)
        when TipoInformacaoComplementar2 like 'FS' then cast(substring(InformacaoComplementar2, 1, 2) as uinteger)
        when TipoInformacaoComplementar3 like 'FS' then cast(substring(InformacaoComplementar3, 1, 2) as uinteger)
        when TipoInformacaoComplementar4 like 'FS' then cast(substring(InformacaoComplementar4, 1, 2) as uinteger)
        when TipoInformacaoComplementar5 like 'FS' then cast(substring(InformacaoComplementar5, 1, 2) as uinteger)
        when TipoInformacaoComplementar6 like 'FS' then cast(substring(InformacaoComplementar6, 1, 2) as uinteger)
    end as funcao,
    case
        when TipoInformacaoComplementar1 like 'FS' then cast(substring(InformacaoComplementar1, 3) as uinteger)
        when TipoInformacaoComplementar2 like 'FS' then cast(substring(InformacaoComplementar2, 3) as uinteger)
        when TipoInformacaoComplementar3 like 'FS' then cast(substring(InformacaoComplementar3, 3) as uinteger)
        when TipoInformacaoComplementar4 like 'FS' then cast(substring(InformacaoComplementar4, 3) as uinteger)
        when TipoInformacaoComplementar5 like 'FS' then cast(substring(InformacaoComplementar5, 3) as uinteger)
        when TipoInformacaoComplementar6 like 'FS' then cast(substring(InformacaoComplementar6, 3) as uinteger)
    end as subfuncao,
    case
        when TipoInformacaoComplementar1 like 'AI' then cast(InformacaoComplementar1 as uinteger)
        when TipoInformacaoComplementar2 like 'AI' then cast(InformacaoComplementar2 as uinteger)
        when TipoInformacaoComplementar3 like 'AI' then cast(InformacaoComplementar3 as uinteger)
        when TipoInformacaoComplementar4 like 'AI' then cast(InformacaoComplementar4 as uinteger)
        when TipoInformacaoComplementar5 like 'AI' then cast(InformacaoComplementar5 as uinteger)
        when TipoInformacaoComplementar6 like 'AI' then cast(InformacaoComplementar6 as uinteger)
    end as codigo_orcamentario
from msc_raw
where remessa = {{remessa}};
