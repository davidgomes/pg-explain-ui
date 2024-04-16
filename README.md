# pg-explain-ui

This is a simple Postgres extension that allows you to easily jump into a visual plan UI for any SQL query.

```sql
explain_ui=# select explain_ui($$SELECT
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
 https://explain.dalibo.com/plan/g1499cc8aa7d1086
(1 row)
```

It is integrated with [Dalibo's Explain UI](https://explain.dalibo.com/) tool, which is a simple service built on top of [pev2](https://github.com/dalibo/pev2) (Postgres Explain Visualizer 2).

This extension is built using [pgrx](https://github.com/pgcentralfoundation/pgrx).

As of now, the extension is not yet published anywhere for consumption.