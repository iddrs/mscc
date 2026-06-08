update msc
set
    saldo_anterior = (
            (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
            where remessa = {{remessa}}
            and msc_raw.chave like msc.chave
            and substring(ContaContabil, 1, 1) in ('1', '3', '5', '7')
            and TipoValor like 'beginning_balance'
            and NaturezaValor like 'D')
        - (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
            where remessa = {{remessa}}
            and msc_raw.chave like msc.chave
            and substring(ContaContabil, 1, 1) in ('1', '3', '5', '7')
            and TipoValor like 'beginning_balance'
            and NaturezaValor like 'C')
    ),
    saldo_atual = (
            (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
            where remessa = {{remessa}}
            and msc_raw.chave like msc.chave
            and substring(ContaContabil, 1, 1) in ('1', '3', '5', '7')
            and TipoValor like 'ending_balance'
            and NaturezaValor like 'D')
        - (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
            where remessa = {{remessa}}
            and msc_raw.chave like msc.chave
            and substring(ContaContabil, 1, 1) in ('1', '3', '5', '7')
            and TipoValor like 'ending_balance'
            and NaturezaValor like 'C')
    )
where remessa = {{remessa}};

update msc
set
    saldo_anterior = (
        (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
         where remessa = {{remessa}}
    and msc_raw.chave like msc.chave
    and substring(ContaContabil, 1, 1) in ('2', '4', '6', '8')
    and TipoValor like 'beginning_balance'
    and NaturezaValor like 'C')
    - (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
where remessa = {{remessa}}
  and msc_raw.chave like msc.chave
  and substring(ContaContabil, 1, 1) in ('2', '4', '6', '8')
  and TipoValor like 'beginning_balance'
  and NaturezaValor like 'D')
    ),
    saldo_atual = (
    (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
where remessa = {{remessa}}
  and msc_raw.chave like msc.chave
  and substring(ContaContabil, 1, 1) in ('2', '4', '6', '8')
  and TipoValor like 'ending_balance'
  and NaturezaValor like 'C')
    - (select coalesce(sum(cast(Valor as decimal(11,2))), 0) from msc_raw
where remessa = {{remessa}}
  and msc_raw.chave like msc.chave
  and substring(ContaContabil, 1, 1) in ('2', '4', '6', '8')
  and TipoValor like 'ending_balance'
  and NaturezaValor like 'D')
    )
where remessa = {{remessa}};

update msc
set
    movimento_debito = (
        coalesce(
            (select sum(cast(Valor as decimal(11,2))) from msc_raw
            where remessa = {{remessa}}
            and msc_raw.chave like msc.chave
            and TipoValor like 'period_change'
            and NaturezaValor like 'D')
        , 0)
    ),
    movimento_credito = (
        coalesce(
            (select sum(cast(Valor as decimal(11,2))) from msc_raw
            where remessa = {{remessa}}
            and msc_raw.chave like msc.chave
            and TipoValor like 'period_change'
            and NaturezaValor like 'C')
        , 0)
    )
where remessa = {{remessa}};

update msc
set
    natureza_saldo_anterior = case
        when substring(conta_contabil, 1, 1) in ('1', '3', '5', '7') and saldo_anterior > 0 then 'D'
        when substring(conta_contabil, 1, 1) in ('1', '3', '5', '7') and saldo_anterior < 0 then 'C'
        when substring(conta_contabil, 1, 1) in ('2', '4', '6', '8') and saldo_anterior > 0 then 'C'
        when substring(conta_contabil, 1, 1) in ('2', '4', '6', '8') and saldo_anterior < 0 then 'D'
        else null
    end,
    natureza_saldo_atual = case
        when substring(conta_contabil, 1, 1) in ('1', '3', '5', '7') and saldo_atual > 0 then 'D'
        when substring(conta_contabil, 1, 1) in ('1', '3', '5', '7') and saldo_atual < 0 then 'C'
        when substring(conta_contabil, 1, 1) in ('2', '4', '6', '8') and saldo_atual > 0 then 'C'
        when substring(conta_contabil, 1, 1) in ('2', '4', '6', '8') and saldo_atual < 0 then 'D'
        else null
    end
where remessa = {{remessa}};