#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Feb 16 13:57:27 2019

@author: Rory
"""

import argparse
'''
This file converts TSV (tab separated files) to input files for MySQL
Afterwards:
    mysql -u <username> -p < <sql_file>
'''


def arguments():
    '''
    Gets the arguments from command line
    '''
    parser = argparse.ArgumentParser(description = "Converts TSV file to MySQL importable file")
    parser.add_argument("TSV_file", type = str, help = "TSV file to be converted (required)")
    parser.add_argument("-o", dest = 'sql_file', type = str, help = "SQL file name for output", required = False)
    parser.add_argument("-t", dest = "Table_name", type = str, help = "Table name for SQL import", required = False)
    args = parser.parse_args()
    return args
    

def transform_tab(data, columns = True, is_char = None):
    '''
    Removes the tabs and replaces them with ","s which is needed for MySQL
    Also adds '`' to the column names
    For data values, adds "'"s to string columns and leaves number data without quotes
    '''
    data = data.replace("\n","")
    if columns:
        data = data.replace("\t","`,`")
        data = "`" + data + "`"
    else:
        if len(is_char) > 0:
            data = data.split("\t")
            for x in is_char:
                data[x] = "'" + data[x] + "'"
            data = ",".join(data)
        else:
            data.replace("\t", ",")
    return data

def is_character(data_types):
    is_char = []
    for x in range(len(data_types)):
        if data_types[x].find("CHAR") > 0:
            is_char.append(x)
    return is_char

def get_data(tsv_file, data_types):
    '''
    Creates the values input for MySQL which is () encapsulated per row with a "," afterwards
    '''
    data = ""
    lines = 1
    is_char = is_character(data_types)
    with open(tsv_file, "r") as tsv:
        tsv.readline()
        for line in tsv:
            lines += 1
            data = data + "(" + transform_tab(line, columns = False, is_char = is_char) + "),\n"
            print(str(lines)+ " lines imported")
    return data[:-2], lines

def get_columns(tsv_file):
    '''
    Gets the column names for MySQL input
    Also finds whether each column is a string or a number
    '''
    with open(tsv_file, "r") as tsv:
        columns = tsv.readline()
        data_types_start = tsv.readline()
    data_types = []
    for value in data_types_start.split("\t"):
        try:
            float(value)
            data_types.append("FLOAT(20)")
        except ValueError:
            data_types.append("VARCHAR(20)")
    columns = transform_tab(columns)
    return columns, data_types
    
def write_file(filename, columns, data_types, data, linecount, table_name):
    '''
    Pulls everything together to create an SQL input file
    '''
    with open(filename, "w+") as file:
        file.write("DROP TABLE IF EXISTS "+ table_name +";\n")
        file.write("CREATE TABLE "+ table_name +"(\n")
        for x in range(len(columns.split(","))):
            if x == (len(columns.split(",")) - 1):
                end = "\n"
            else:
                end = ",\n"
            file.write(columns.split(",")[x] + " "+ data_types[x] +" DEFAULT NULL" + end)
        file.write(");")
        file.write("\n\nINSERT INTO "+ table_name +"\n(")
        file.write(columns)
        file.write(")\nVALUES\n")
        file.write(data)    
        file.write(";")
        
def run():
    '''
    Runs all the functions above together
    '''
    args = arguments()
    columns, data_types = get_columns(args.TSV_file)
    print("Column names imported")
    data, linecount = get_data(args.TSV_file, data_types)
    if args.sql_file is not None:
        filename = args.sql_file
    else:
        filename = args.tsv_file[:args.TSV_file.upper().find(".TSV")] + ".sql"
    if args.Table_name is not None:
        table_name = args.Table_name
    else:
        table_name = args.tsv_file[:args.TSV_file.upper().find(".TSV")]
    print("Writing")
    write_file(filename, columns, data_types, data, linecount, table_name)
    
if __name__ == "__main__":
    run()
