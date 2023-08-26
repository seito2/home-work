-- Add migration script here
create table homeworks (
    id int,
    display_name text not null,
    description text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    primary key (id)
);

create table schedules (
    id int,
    start_date timestamp not null,
    end_date timestamp not null,
    cron text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    primary key (id)
);

create table homeworks_schedules (
    homework_id int not null references homeworks(id),
    schedule_id int not null references schedules(id),
    primary key (homework_id, schedule_id)
);
