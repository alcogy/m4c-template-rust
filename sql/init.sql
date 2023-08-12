CREATE TABLE product (
  id serial primary key,
  name varchar(255) not null default '',
  price integer not null default 0,
  cost integer not null default 0,
  tax_rate smallint not null default 0
);

INSERT INTO product (
  id,
  name,
  price,
  cost,
  tax_rate
) VALUES (
  default,
  'Laptop',
  300000,
  220000,
  10
);

INSERT INTO product (
  id,
  name,
  price,
  cost,
  tax_rate
) VALUES (
  default,
  'Mother Board',
  25000,
  18000,
  10
);

INSERT INTO product (
  id,
  name,
  price,
  cost,
  tax_rate
) VALUES (
  default,
  'CPU',
  85000,
  60000,
  10
);
