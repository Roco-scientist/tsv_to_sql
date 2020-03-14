#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sat Feb 16 13:57:27 2019

@author: Rory

This file converts TSV (tab separated files) to input files for MySQL
Afterwards:
    mysql -u <username> -p < <sql_file>
"""

import argparse
import csv
import sys

from typing import List, Tuple


def arguments():
    '''
    Gets the arguments from command line
    '''
    parser = argparse.ArgumentParser(
        description="Converts TSV file to MySQL importable file",
        usage="python %(prog)s [-options] <TSV_file>")
    parser.add_argument("TSV_file", type=str, help="file.tsv file to be converted (required)")
    parser.add_argument("-o", dest='sql_file', type=str,
                        help="SQL file name for output (default file.sql)", required=False)
    parser.add_argument("-t", dest="Table_name", type=str,
                        help="Table name for SQL import (default file)", required=False)
    parser.add_argument("-p", dest="primary_first", action="store_true", default=False,
                        help="First column used as a primary key (default False)", required=False)
    args = parser.parse_args()
    return args


def transform_tab(data: str) -> str:
    '''
    Removes the tabs and replaces them with ","s which is needed for MySQL
    Also adds '`' to the column names
    For data values, adds "'"s to string columns and leaves number data without quotes
    :data: 
    :return: reformed data
    '''
    data = data.replace("\n", "")
    data = data.replace("\t", "`,`")
    data = f"`{data}`"
    return data


def is_character(data_types: List[str]) -> List[int]:
    """
    :data_types:
    :return:
    """
    is_char = []
    for x in range(len(data_types)):
        if data_types[x].find("CHAR"):
            is_char.append(x)
    return is_char


def add_quotes(row: List[str], is_char: List[int]) -> List[str]:
    """
    Adds quotes around string values in rows
    :row: a single row from the tsv file
    :is_char: a list of indexes that are texts instead of numbers
    :return: row with quotes around string values
    """
    for x in is_char:
        row[x] = f"'{row[x]}'"
    return row


def get_data(tsv_file: str, data_types: List[str]) -> Tuple[str, int]:
    '''
    Creates the values input for MySQL which is () encapsulated per row with a "," afterwards
    :tsv_file:
    :data_types:
    :return:
    '''
    is_char = is_character(data_types)
    with open(tsv_file, "r") as tsv:
        tsvreader = csv.reader(tsv, delimiter="\t")
        tsv_data = list(tsvreader)
    tsv_data = tsv_data[1:]
    for x, row in enumerate(tsv_data):
        if len(is_char):
            row = add_quotes(row, is_char)
        tsv_data[x] = ",".join(row)
    data = "),\n(".join(tsv_data)
    data = f"({data})"
    return data, len(tsv_data)


def get_columns(tsv_file: str) -> Tuple[str, List[str]]:
    '''
    Gets the column names for MySQL input
    Also finds whether each column is a string or a number
    :tsv_file:
    :return:
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


def write_file(filename: str, columns: str, data_types: List[str], data: str, linecount: int,
               table_name: str, primary_first):
    '''
    Pulls everything together to create an SQL input file
    :filename:
    :columns:
    :data_types:
    :data: refurmed data into sql format for import
    :linecount: number of lines in TSV file/database
    :table_name:
    :primary_first:
    :return:
    '''
    with open(filename, "w+") as file:
        file.write(f"DROP TABLE IF EXISTS {table_name};\n")
        file.write(f"CREATE TABLE {table_name}(\n")
        for x, column in enumerate(columns.split(",")):
            if x == 0:
                if primary_first:
                    file.write(f"{column} {data_types[x]} NOT NULL,\n")
                else:
                    file.write("id INT NOT NULL AUTO_INCREMENT,\n")
                    file.write(f"{column} {data_types[x]},\n")
            else:
                file.write(f"{column} {data_types[x]},\n")
        if primary_first:
            file.write(f"PRIMARY KEY ( {columns.split(',')[0]} )")
        else:
            file.write("PRIMARY KEY ( id )")
        file.write(");")
        file.write(f"\n\nINSERT INTO {table_name}\n(")
        file.write(columns)
        file.write(")\nVALUES\n")
        file.write(data)
        file.write(";")


def main():
    args = arguments()
    columns, data_types = get_columns(args.TSV_file)
    print("Column names imported")
    data, linecount = get_data(args.TSV_file, data_types)
    if args.sql_file is not None:
        filename = args.sql_file
    else:
        filename = args.TSV_file[:args.TSV_file.upper().find(".TSV")] + ".sql"
    if args.Table_name is not None:
        table_name = args.Table_name
    else:
        table_name = args.TSV_file[:args.TSV_file.upper().find(".TSV")]
    print("Writing")
    write_file(filename, columns, data_types, data, linecount, table_name, args.primary_first)


if __name__ == "__main__":
    if float(f"{sys.version_info[0]}.{sys.version_info[1]}") < 3.6:
        raise SystemError("Python 3.6 or newer required")
    main()
