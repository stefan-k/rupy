#!/usr/bin/python
"""
fu
"""

import numpy
import rupy


def blah():
    """blah"""
    print("fu")


def blah2(num):
    """blah"""
    print(num)


rupy.closure(blah)
rupy.closure2(blah2)
