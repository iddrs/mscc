update msc
set
    entidade = case orgao
        when 131 then 'pm'
        when 132 then 'fpsm'
        when 231 then 'cm'
    end
where remessa = {{remessa}};