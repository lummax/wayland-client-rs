# Copyright (c) <2015> <lummax>
# Licensed under MIT (http://opensource.org/licenses/MIT)

from collections import namedtuple

Protocol = namedtuple('Protocol', 'name, copyright, interfaces')
Interface = namedtuple('Interface', 'name, version, description, requests, events, enums')
Request = namedtuple('Requests', 'name, type, description, arguments, since')
Event = namedtuple('Event', 'name, description, arguments, since')
Enum = namedtuple('Enum', 'name, description, entries')
Description = namedtuple('Description', 'summary, text')
Argument = namedtuple('Argument', 'name, type, interface, summary, allow_null')
Entry = namedtuple('Entry', 'name, value, summary')

def parse_protocol(node):
    copy = node.find('copyright')
    return Protocol(node.get('name', ''),
                    copy.text.strip() if not copy is None else '',
                    list(map(parse_interface, node.iter('interface'))))

def parse_interface(node):
    return Interface(node.get('name', ''),
                     node.get('version', ''),
                     parse_description(node.find('description')),
                     list(map(parse_request, node.iter('request'))),
                     list(map(parse_event, node.iter('event'))),
                     list(map(parse_enum, node.iter('enum'))))

def parse_request(node):
    return Request(node.get('name', ''),
                   node.get('type', ''),
                   parse_description(node.find('description')),
                   list(map(parse_argument, node.iter('arg'))),
                   node.get('since', ''))

def parse_event(node):
    return Event(node.get('name', ''),
                 parse_description(node.find('description')),
                 list(map(parse_argument, node.iter('arg'))),
                 node.get('since', ''))

def parse_enum(node):
    return Enum(node.get('name', ''),
                parse_description(node.find('description')),
                list(map(parse_entry, node.iter('entry'))))

def parse_argument(node):
    return Argument(node.get('name', ''),
                    node.get('type', ''),
                    node.get('interface', ''),
                    node.get('summary', ''),
                    node.get('allow_null', 'false'))

def parse_entry(node):
    return Entry(node.get('name', ''),
                 node.get('value', ''),
                 node.get('summary', ''))

def parse_description(node):
    if node is None: return Description('', '')
    summary = node.attrib['summary']
    text = node.text
    return Description(summary if summary else '',
                       text.strip() if text else '')
