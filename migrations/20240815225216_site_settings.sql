drop table if exists site;
create table site (
    slug text primary key not null,
    title text not null,
    home_page text references pages(slug)
);

delete from site;
insert into site values ('sample-site', 'Sample Site', 'about');
