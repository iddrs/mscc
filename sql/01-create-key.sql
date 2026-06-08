update msc_raw
set
    chave = remessa
            || ContaContabil
            || coalesce(InformacaoComplementar1, '0')
            || coalesce(TipoInformacaoComplementar1, '0')
            || coalesce(InformacaoComplementar2, '0')
            || coalesce(TipoInformacaoComplementar2, '0')
            || coalesce(InformacaoComplementar3, '0')
            || coalesce(TipoInformacaoComplementar3, '0')
            || coalesce(InformacaoComplementar4, '0')
            || coalesce(TipoInformacaoComplementar4, '0')
            || coalesce(InformacaoComplementar5, '0')
            || coalesce(TipoInformacaoComplementar5, '0')
            || coalesce(InformacaoComplementar6, '0')
            || coalesce(TipoInformacaoComplementar6, '0')
