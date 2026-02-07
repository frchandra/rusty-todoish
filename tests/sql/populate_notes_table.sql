INSERT INTO notes (id, title, content, is_published)
SELECT
    gen_random_uuid()::text,
    'Note ' || gs || ' - ' || substr(md5(random()::text), 1, 6),
    'Random content for note #' || gs || '. Lorem ipsum ' || substr(md5(random()::text), 1, 12),
    random() < 0.5
FROM generate_series(1, 10) AS gs;