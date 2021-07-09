create table tasks (
  _id int8 primary key generated always as identity,
  description varchar not null
)
