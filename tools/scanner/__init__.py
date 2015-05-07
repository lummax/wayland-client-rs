# Copyright (c) <2015> <lummax>
# Licensed under MIT (http://opensource.org/licenses/MIT)

import xml.etree.ElementTree as etree

import scanner.parser
import scanner.context
from scanner.generator import client
from scanner.version import __version__

def build_ast(path):
    root = etree.parse(path).getroot()
    ast = scanner.parser.parse_protocol(root)
    return scanner.context.context_protocol(ast)

def generate_client_protocol_bindings(ast):
   yield from client.generate_bindings(ast)
