# pg-explain-ui

This is a simple Postgres extension that allows you to easily jump into a visual plan UI for any SQL query.

```sql
CREATE TABLE authors (
    author_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE publishers (
    publisher_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE categories (
    category_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE books (
    book_id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author_id INT NOT NULL REFERENCES authors(author_id),
    publisher_id INT NOT NULL REFERENCES publishers(publisher_id),
    publication_date DATE
);

CREATE TABLE book_categories (
    book_id INT NOT NULL REFERENCES books(book_id),
    category_id INT NOT NULL REFERENCES categories(category_id),
    PRIMARY KEY (book_id, category_id)
);

SELECT explain_ui($$SELECT
    b.title AS "Book Title",
    a.name AS "Author",
    p.name AS "Publisher",
    array_agg(c.name) AS "Categories",
    b.publication_date AS "Publication Date",
    COUNT(c.category_id) AS "Number of Categories"
FROM
    books b
INNER JOIN authors a ON b.author_id = a.author_id
INNER JOIN publishers p ON b.publisher_id = p.publisher_id
INNER JOIN book_categories bc ON b.book_id = bc.book_id
INNER JOIN categories c ON bc.category_id = c.category_id
WHERE
    p.name != 'Test'
GROUP BY
    b.book_id, a.name, p.name, b.publication_date
HAVING
    COUNT(c.category_id) > 1
ORDER BY
    b.publication_date DESC;
$$);
                    explain_ui
--------------------------------------------------
 https://explain.dalibo.com/plan/ccg2e5fedd913bb7
(1 row)
```

It is integrated with [Dalibo's Explain UI](https://explain.dalibo.com/) tool, which is a simple service built on top of [pev2](https://github.com/dalibo/pev2) (Postgres Explain Visualizer 2).

This extension is built using [pgrx](https://github.com/pgcentralfoundation/pgrx).

As of now, the extension is not yet published anywhere for consumption.