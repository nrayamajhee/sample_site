drop table if exists pages;
create table pages (
    slug text primary key not null,
    title text not null,
    content text not null
);
