create table if not exists models 
(
    id integer primary key not null, 
    name varchar(512) not null,
    model_type varchar(128) not null,
    text blob not null,
    updated datetime not null default current_timestamp,
    created datetime not null default current_timestamp    
);
create unique index if not exists models_name_unique_index on models(name);