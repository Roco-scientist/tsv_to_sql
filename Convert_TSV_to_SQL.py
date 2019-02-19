#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Feb 16 13:57:27 2019

@author: Rory
"""

import argparse
import csv
'''
This file converts TSV (tab separated files) to input files for MySQL
Afterwards:
    mysql -u <username> -p < <sql_file>
'''


def arguments():
    '''
    Gets the arguments from command line
    '''
    parser = argparse.ArgumentParser(description = "Converts TSV file to MySQL importable file", usage = "python %(prog)s [-options] <TSV_file>")
    parser.add_argument("TSV_file", type = str, help = "file.tsv file to be converted (required)")
    parser.add_argument("-o", dest = 'sql_file', type = str, help = "SQL file name for output (default file.sql)", required = False)
    parser.add_argument("-t", dest = "Table_name", type = str, help = "Table name for SQL import (default file)", required = False)
    parser.add_argument("-p", dest = "primary_first", action = "store_true", default = False, help = "First column used as a primary key (default False)", required = False)
    args = parser.parse_args()
    return args
    

def transform_tab(data):
    '''
    Removes the tabs and replaces them with ","s which is needed for MySQL
    Also adds '`' to the column names
    For data values, adds "'"s to string columns and leaves number data without quotes
    '''
    data = data.replace("\n","")
    data = data.replace("\t","`,`")
    data = "`" + data + "`"
    return data

def is_character(data_types):
    is_char = []
    for x in range(len(data_types)):
        if data_types[x].find("CHAR") > 0:
            is_char.append(x)
    return is_char

def add_quotes(row, is_char):
    for x in is_char:
        row[x] = "'" + row[x] + "'"
    return row

def get_data(tsv_file, data_types):
    '''
    Creates the values input for MySQL which is () encapsulated per row with a "," afterwards
    '''
    tsv_data = []
    is_char = is_character(data_types)
    with open(tsv_file, "r") as tsv:
        tsvreader = csv.reader(tsv, delimiter = "\t")
        for row in tsvreader:
            tsv_data.append(row)
    tsv_data = tsv_data[1:]
    for x in range(len(tsv_data)):
        if len(is_char) > 0:
            tsv_data[x] = add_quotes(tsv_data[x], is_char)
        tsv_data[x] = ",".join(tsv_data[x])
    data = "),\n(".join(tsv_data)
    data = "(" + data + ")"
    return data, len(tsv_data)

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
            data_types.append("FLOAT")
        except ValueError:
            data_types.append("VARCHAR(20)")
    columns = transform_tab(columns)
    return columns, data_types
    
def write_file(filename, columns, data_types, data, linecount, table_name, primary_first):
    '''
    Pulls everything together to create an SQL input file
    '''
    with open(filename, "w+") as file:
        file.write("DROP TABLE IF EXISTS "+ table_name +";\n")
        file.write("CREATE TABLE "+ table_name +"(\n")
        for x in range(len(columns.split(","))):
            if x == 0:
                if primary_first:
                    file.write(columns.split(",")[x] + " "+ data_types[x] +" NOT NULL,\n")
                else:
                    file.write("id INT NOT NULL AUTO_INCREMENT,\n")
                    file.write(columns.split(",")[x] + " "+ data_types[x] + ",\n")
            else:
                file.write(columns.split(",")[x] + " "+ data_types[x] + ",\n")
        if primary_first:
            file.write("PRIMARY KEY ( "+ columns.split(",")[0] +" )")
        else:
            file.write("PRIMARY KEY ( id )")
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
    write_file(filename, columns, data_types, data, linecount, table_name, args.primary_first)
    
if __name__ == "__main__":
    run()
