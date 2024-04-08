# pg-explain-ui

This is a simple Postgres extension that allows you to easily jump into a visual plan UI for any SQL query.

```sql
explain_ui=# select explain_ui('SELECT
explain_ui'#     b.title AS "Book Title",
explain_ui'#     a.name AS "Author",
explain_ui'#     p.name AS "Publisher",
explain_ui'#     array_agg(c.name) AS "Categories",
explain_ui'#     b.publication_date AS "Publication Date",
explain_ui'#     COUNT(c.category_id) AS "Number of Categories"
explain_ui'# FROM
explain_ui'#     books b
explain_ui'# INNER JOIN authors a ON b.author_id = a.author_id
explain_ui'# INNER JOIN publishers p ON b.publisher_id = p.publisher_id
explain_ui'# INNER JOIN book_categories bc ON b.book_id = bc.book_id
explain_ui'# INNER JOIN categories c ON bc.category_id = c.category_id
explain_ui'# WHERE
explain_ui'#     p.name != ''Test''
explain_ui'# GROUP BY
explain_ui'#     b.book_id, a.name, p.name, b.publication_date
explain_ui'# HAVING
explain_ui'#     COUNT(c.category_id) > 1
explain_ui'# ORDER BY
explain_ui'#     b.publication_date DESC;
explain_ui'# ');
                    explain_ui
--------------------------------------------------
 https://explain.dalibo.com/plan/g1499cc8aa7d1086
(1 row)
```

It is integrated with [Dalibo's Explain UI](https://explain.dalibo.com/) tool, which is a simple service built on top of [pev2](https://github.com/dalibo/pev2) (Postgres Explain Visualizer 2).

This extension is built using [pgrx](https://github.com/pgcentralfoundation/pgrx).

As of now, the extension is not yet published anywhere for consumption.