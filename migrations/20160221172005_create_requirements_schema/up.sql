CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

CREATE TABLE programs (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    org_id INTEGER REFERENCES organizations(id)
);

CREATE TYPE requirement_field AS ENUM ('age', 'county', 'children_count', 'income', 'single_parent');
CREATE TYPE requirement_type AS ENUM ('boolean', 'int_range', 'int_equals', 'string_equals');

CREATE TABLE requirements (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    field requirement_field NOT NULL,
    req_type requirement_type NOT NULL,
    req_args TEXT ARRAY NOT NULL
);

CREATE TYPE checklist_type AS ENUM ('or', 'and', 'req');

CREATE TABLE checklists (
    id SERIAL PRIMARY KEY,
    program_id INTEGER REFERENCES programs(id),
    check_type checklist_type NOT NULL,
    checklist_id INTEGER REFERENCES checklists(id),
    requirements_id INTEGER REFERENCES requirements(id)
);
