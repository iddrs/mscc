INSTALL excel;
LOAD excel;

create table if not exists pcasp as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'pcasp');

create table if not exists po as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'po');

create table if not exists funcao as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'funcao');

create table if not exists subfuncao as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'subfuncao');

create table if not exists ndo as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'ndo');

create table if not exists nro as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'nro');

create table if not exists exercicio_fr as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'exercicio_fr');

create table if not exists fr as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'fr');

create table if not exists co as
select * from read_xlsx('external_data.xlsx', header = true, sheet = 'co');