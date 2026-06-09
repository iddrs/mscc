INSTALL excel;
LOAD excel;

create table if not exists msc_pcasp as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'pcasp');

create table if not exists msc_po as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'po');

create table if not exists msc_funcao as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'funcao');

create table if not exists msc_subfuncao as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'subfuncao');

create table if not exists msc_ndo as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'ndo');

create table if not exists msc_nro as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'nro');

create table if not exists msc_exercicio_fr as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'exercicio_fr');

create table if not exists msc_fr as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'fr');

create table if not exists msc_co as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'co');