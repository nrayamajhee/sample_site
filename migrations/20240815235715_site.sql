drop table if exists site;
create table site (
    slug text primary key not null,
    title text not null,
    home_page text references pages(slug)
);

delete from site;
insert into site values ('sample-site', 'Sample Site', 'about');

drop table if exists pages;
create table pages (
    slug text primary key not null,
    title text not null,
    content text not null
    menu_order integer not null 
    show_in_menu boolean not null default true
);
delete from pages;
insert into pages values ('about', 'About', E'# About\n\nThis is about page.\n', 0, true);
insert into pages values ('contact', 'Contact', E'# Contact\n\nThis is contact page.');

