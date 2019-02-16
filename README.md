# tsv_to_sql
TSV to SQL for MySQL

Use:\n
python Convert_TSV_to_SQL.py [-o <SQL_file>][-t <table_name>] <TSV_file>

Then to import:\n
mysql -u <user_name> -p <dataframe> < <SQL_file>
