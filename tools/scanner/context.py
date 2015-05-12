# Copyright (c) <2015> <lummax>
# Licensed under MIT (http://opensource.org/licenses/MIT)

from collections import namedtuple
from scanner import parser

Interface = namedtuple('Interface', 'name, wl_name, version, description, requests, events, enums')
EmptyRequest = namedtuple('EmptyRequest', 'name, is_destructor, method_name, description, arguments, since, opcode')
FactoryRequest = namedtuple('FactoryRequest', 'name, method_name, interface, description, arguments, since, opcode')
DynamicRequest = namedtuple('DynamicRequest', 'name, method_name, description, arguments, since, opcode')
Event = namedtuple('Event', 'name, wl_name, description, arguments, since, opcode')
Enum = namedtuple('Enum', 'name, wl_name, description, entries')
Argument = namedtuple('Argument', 'name, type, interface, wl_interface, summary, allow_null')
Entry = namedtuple('Entry', 'name, wl_name, value, summary')

def de_keyword_ize(name):
    keywords = ["as", "break", "crate", "else", "enum", "extern", "false",
                "fn", "for", "if", "impl", "in", "let", "loop", "match",
                "mod", "move", "mut", "pub", "ref", "return", "static",
                "self", "Self", "struct", "super", "true", "trait", "type",
                "unsafe", "use", "virtual", "while", "continue", "box",
                "const", "where", "proc", "alignof", "become", "offsetof",
                "priv", "pure", "sizeof", "typeof", "unsized", "yield", "do",
                "abstract", "final", "override", "macro"]
    name = name.replace('wl_', '')
    if name in keywords: return name + '_'
    elif name.isdigit(): return '_' + name
    else: return name

def classify_name(name):
    name = name.replace('wl_', '')
    classified = ''.join(map(str.capitalize, name.split('_')))
    return de_keyword_ize(classified)

def context_protocol(node):
    return parser.Protocol(name=node.name,
                           copyright=node.copyright,
                           interfaces=list(map(context_interface,
                                               node.interfaces)))

def context_interface(node):
    requests = list()
    for (num, request) in enumerate(node.requests):
        requests.append(context_request(request, de_keyword_ize(node.name), num))
    events = list()
    for (num, event) in enumerate(node.events):
        events.append(context_event(event, de_keyword_ize(node.name), num))
    enums = list()
    for enum in node.enums:
        enums.append(context_enum(enum, de_keyword_ize(node.name)))

    return Interface(name=classify_name(node.name),
                     wl_name=de_keyword_ize(node.name),
                     version=node.version,
                     description=node.description,
                     requests=requests,
                     events=events,
                     enums=enums)

def context_request(node, interface, num):
    special_args = list(filter(lambda a : a.type == 'new_id', node.arguments))
    normal_args = list(filter(lambda a : a.type != 'new_id', node.arguments))
    special_arg = special_args[0] if special_args else None

    if node.type == 'destructor' or special_arg is None:
        return EmptyRequest(name=classify_name(node.name),
                            is_destructor=node.type == 'destructor',
                            method_name=de_keyword_ize(node.name),
                            description=node.description,
                            arguments=list(map(context_argument, node.arguments)),
                            since=node.since,
                            opcode=str(num))
    elif not special_arg.interface:
        return DynamicRequest(name=classify_name(node.name),
                              method_name=de_keyword_ize(node.name),
                              description=node.description,
                              arguments=list(map(context_argument, normal_args)),
                              since=node.since,
                              opcode=str(num))
    else:
        return FactoryRequest(name=classify_name(node.name),
                              method_name=de_keyword_ize(node.name),
                              interface=classify_name(special_arg.interface),
                              description=node.description,
                              arguments=list(map(context_argument, normal_args)),
                              since=node.since,
                              opcode=str(num))


def context_event(node, interface, num):
    return Event(name=classify_name(node.name),
                 wl_name=de_keyword_ize(node.name),
                 description=node.description,
                 arguments=list(map(context_argument, node.arguments)),
                 since=node.since,
                 opcode=str(num))

def context_argument(node):
    return Argument(name=de_keyword_ize(node.name),
                    type=node.type,
                    interface=classify_name(node.interface),
                    wl_interface=node.interface,
                    summary=node.summary,
                    allow_null=node.allow_null == 'true')

def context_enum(node, interface):
    return Enum(name=classify_name(interface) + classify_name(node.name),
                wl_name=de_keyword_ize(node.name),
                description=node.description,
                entries=list(map(context_entry, node.entries)))

def context_entry(node):
    return Entry(name=classify_name(node.name),
                 wl_name=de_keyword_ize(node.name),
                 value=node.value,
                 summary=node.summary)
