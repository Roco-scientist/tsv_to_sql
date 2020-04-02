# tsv_to_sql
TSV to SQL for MySQL

Use Python:<br>
`python Convert_TSV_to_SQL.py [- options] <TSV_file>`
<br>
Use Rust:<br>
`tsv_csv_to_sql -f <file.tsv> -t <table_name> -o <output_sql_file>`

Then to import:<br>
`mysql -u <user_name> -p <database_name> < <SQL_file>`
