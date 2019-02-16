# tsv_to_sql
TSV to SQL for MySQL

Use:
python Convert_TSV_to_SQL.py [-o <SQL_file>][-t <table_name>] <TSV_file>

Then to import:
mysql -u <username> -p <dataframe> < <SQL_file>
