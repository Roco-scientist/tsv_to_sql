#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import unittest
import convert_TSV_to_SQL


class TestTransformTab(unittest.TestCase):

    """Tests convert_TSV_to_SQL functions"""
    tsv_data = ["one\ttwo\tthree",
                "12\tyellow\t34",
                "56\tred\t78"]
    data_types = ["FLOAT", "VARCHAR(20)", "FLOAT"]
    data = "(12,'yellow',34),\n(56,'red',78)"
    linecount = 2

    def test_get_column_headers(self):
        self.assertEqual(convert_TSV_to_SQL.transform_tab(self.tsv_data[0]),
                         "`one`,`two`,`three`")
        self.assertEqual(convert_TSV_to_SQL.get_column_headers(self.tsv_data),
                         ("`one`,`two`,`three`", self.data_types))

    def test_get_data(self):
        self.assertEqual(convert_TSV_to_SQL.get_data(self.tsv_data, self.data_types),
                         (self.data, self.linecount))


if __name__ == "__main__":
    unittest.main()
