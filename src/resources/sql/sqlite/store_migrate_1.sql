create table if not exists models 
(
    id integer primary key not null, 
    name varchar(512) not null,
    text blob not null  
);
create unique index models_name_unique_index on models(name);