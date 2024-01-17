drop table if exists pages;
create table pages (
    slug text primary key not null,
    title text not null,
    content text not null
);
delete from pages;
insert into pages values ('about', 'About', E'# About\n\nThis is about page.\n');
insert into pages values ('contact', 'Contact', E'# Contact\n\nThis is contact page.');

