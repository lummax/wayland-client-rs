# Copyright (c) <2015> <lummax>
# Licensed under MIT (http://opensource.org/licenses/MIT)

from collections import namedtuple
import pystache

import scanner

def render(template, context=None):
    renderer = pystache.Renderer(escape=lambda u: u)
    return renderer.render(template, context)

def generate_bindings(protocol):
    copyright = '\n'.join('// ' + line.strip() for line in protocol.copyright.splitlines())

    yield ('mod.rs', generate_mod_file(protocol))
    for interface in protocol.interfaces:
        filename = interface.wl_name + '.rs'
        content = generate_file(interface, copyright)
        yield (filename, content)

def process_description(des):
    if hasattr(des, 'text'): lines = des.text.splitlines()
    else: lines = des.splitlines()
    return ({'line': '/// ' + line.strip()} for line in lines)

MOD_TEMPLATE = '''// Copyright (c) <2015> <lummax>
// Licensed under MIT (http://opensource.org/licenses/MIT)

// Generated with version {{version}}

{{#modules}}
mod {{module}};
{{/modules}}

{{#modules}}
pub use self::{{module}}::{ {{exports}} };
{{/modules}}

use ffi;

pub trait FromPrimitive {
    fn from_u32(num: u32) -> Option<Self>;
}

pub trait GetInterface {
    fn get_interface() -> *const ffi::wayland::WLInterface;
}'''

def generate_mod_file(protocol):
    modules = list()
    for interface in protocol.interfaces:
        exports = [interface.name]
        if interface.events: exports.append(interface.name + 'EventHandler')
        exports.extend(enum.name for enum in interface.enums)
        modules.append({
            'module': interface.wl_name,
            'exports': ', '.join(exports),
        })
    data = {
        'modules': modules,
        'version': scanner.__version__,
    }
    return render(MOD_TEMPLATE, data)



FILE_TEMPLATE = '''{{copyright}}

// Generated with version {{version}}

#![allow(unused_imports)]

use std::{ptr, mem};
use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use libc::{c_void, c_int, uint32_t};

use ffi;
use client::protocol::{FromPrimitive, GetInterface};
{{#is_display}}
use client::base::Display as BaseDisplay;
{{/is_display}}
{{^is_display}}
use client::base::Proxy as BaseProxy;
{{/is_display}}
use client::base::{FromRawPtr, AsRawPtr, EventQueue};

#[link(name="wayland-client")]
extern {
    static wl_{{wl_name}}_interface: ffi::wayland::WLInterface;
}
{{#enums}}
{{enum}}
{{/enums}}
{{event_enum}}
{{request_enum}}
{{struct}}
{{event_dispatcher}}
{{event_handler}}'''

def generate_file(interface, copyright):
    data = {
        'copyright': copyright,
        'is_display': interface.wl_name == 'display',
        'wl_name': interface.wl_name,
        'enums': ({'enum': generate_enum(enum)} for enum in interface.enums),
        'event_enum': generate_event_request_enum(interface.name + 'Event', interface.events),
        'request_enum': generate_event_request_enum(interface.name + 'Request', interface.requests),
        'struct': generate_struct(interface),
        'event_dispatcher': generate_event_dispatcher(interface),
        'event_handler': generate_event_handler(interface),
        'version': scanner.__version__,
    }
    return render(FILE_TEMPLATE, data)

ENUM_TEMPLATE = '''
{{#description}}
{{line}}
{{/description}}
#[repr(C)]
{{#is_pub}}
#[derive(Debug)]
{{/is_pub}}
{{#is_pub}}pub {{/is_pub}}enum {{name}} {
    {{#entries}}
    {{#entry_description}}
    {{line}}
    {{/entry_description}}
    {{#entry_value}}
    {{entry_name}} = {{entry_value}},
    {{/entry_value}}
    {{^entry_value}}
    {{entry_name}},
    {{/entry_value}}
    {{/entries}}
}

impl FromPrimitive for {{name}} {
    fn from_u32(num: u32) -> Option<Self> {
        return match num {
            {{#entries}}
            {{#entry_value}}
            {{entry_value}} => Some({{name}}::{{entry_name}}),
            {{/entry_value}}
            {{/entries}}
            _ => None
        }
    }
}'''

def generate_enum(enum):
    entries = list({'entry_description': process_description(entry.summary),
                    'entry_name': entry.name,
                    'entry_value': entry.value} for entry in enum.entries)
    if len(enum.entries) == 1:
        entries.append({
            'entry_description': 'Needed due to a bug in rustc',
            'entry_name': '_Dummy',
        })
    data = {
        'description': process_description(enum.description),
        'is_pub': True,
        'name': enum.name,
        'entries': entries,
    }
    return render(ENUM_TEMPLATE, data)

def generate_event_request_enum(name, items):
    if not items: return ''
    entries = list({'entry_name': item.name,
                    'entry_value': item.opcode} for item in items)
    if len(items) == 1:
        entries.append({
            'entry_name': '_Dummy',
        })
    data = {
        'name': name,
        'entries': entries,
    }
    return render(ENUM_TEMPLATE, data)

STRUCT_TEMPLATE = '''
{{#description}}
{{line}}
{{/description}}
#[derive(Debug)]
pub struct {{name}} {
    {{#is_display}}
    display: BaseDisplay,
    {{/is_display}}
    {{^is_display}}
    proxy: BaseProxy,
    {{/is_display}}
}

impl {{name}} {
    {{#methods}}
    {{method}}
    {{/methods}}

    {{#is_display}}
    pub fn connect(name: &str) -> Result<Self, &'static str> {
        return match BaseDisplay::connect(name) {
            Ok(display) => Ok(Display{
                display: display,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn connect_to_fd(fd: RawFd) -> Result<Self, &'static str> {
        return match BaseDisplay::connect_to_fd(fd) {
            Ok(display) => Ok(Display{
                display: display,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn get_id(&mut self) -> u32 {
        return self.display.get_id();
    }

    pub fn get_class(&mut self) -> String {
        return self.display.get_class();
    }

    pub fn set_queue(&mut self, queue: &mut EventQueue) {
        self.display.set_queue(queue);
    }

    pub fn get_fd(&mut self) -> RawFd {
        return self.display.get_fd();
    }

    pub fn dispatch(&mut self) -> Option<u32> {
        return self.display.dispatch();
    }

    pub fn dispatch_queue(&mut self, queue: &mut EventQueue) -> Option<u32> {
        return self.display.dispatch_queue(queue);
    }

    pub fn dispatch_queue_pending(&mut self, queue: &mut EventQueue) -> Option<u32> {
        return self.display.dispatch_queue_pending(queue);
    }

    pub fn dispatch_pending(&mut self) -> Option<u32> {
        return self.display.dispatch_pending();
    }

    pub fn get_error(&mut self) -> i32 {
        return self.display.get_error();
    }

    pub fn flush(&mut self) -> Option<u32> {
        return self.display.flush();
    }

    pub fn roundtrip_queue(&mut self, queue: &mut EventQueue) -> Option<u32> {
        return self.display.roundtrip_queue(queue);
    }

    pub fn roundtrip(&mut self) -> Option<u32> {
        return self.display.roundtrip();
    }

    pub fn create_queue(&mut self) -> Result<EventQueue, &'static str> {
        return self.display.create_queue();
    }

    pub fn prepare_read_queue(&mut self, queue: &mut EventQueue) -> bool {
        return self.display.prepare_read_queue(queue);
    }

    pub fn prepare_read(&mut self) -> bool {
        return self.display.prepare_read();
    }

    pub fn cancel_read(&mut self) {
        self.display.cancel_read();
    }

    pub fn read_events(&mut self) -> bool {
        return self.display.read_events();
    }
    {{/is_display}}
    {{^is_display}}
    pub fn get_id(&mut self) -> u32 {
        return self.proxy.get_id();
    }

    pub fn get_class(&mut self) -> String {
        return self.proxy.get_class();
    }

    pub fn set_queue(&mut self, queue: &mut EventQueue) {
        self.proxy.set_queue(queue);
    }
    {{/is_display}}
}

{{#is_display}}
impl AsRawPtr<ffi::wayland::WLDisplay> for {{name}} {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLDisplay {
        return self.display.as_mut_ptr();
    }
}
{{/is_display}}

{{^is_display}}
impl FromRawPtr<ffi::wayland::WLProxy> for {{name}} {
    fn from_mut_ptr(ptr: *mut ffi::wayland::WLProxy) -> Result<Self, &'static str> {
        return match FromRawPtr::from_mut_ptr(ptr) {
            Ok(proxy) => Ok({{name}} {
                proxy: proxy,
            }),
            Err(str) => Err(str),
        }
    }
}

impl AsRawPtr<ffi::wayland::WLProxy> for {{name}} {
    fn as_mut_ptr(&mut self) -> *mut ffi::wayland::WLProxy {
        return self.proxy.as_mut_ptr();
    }
}
{{/is_display}}

impl GetInterface for {{name}} {
    fn get_interface() -> *const ffi::wayland::WLInterface {
        return &wl_{{wl_name}}_interface as *const ffi::wayland::WLInterface;
    }
}'''

def generate_struct(interface):
    data = {
        'description': process_description(interface.description),
        'is_display': interface.wl_name == 'display',
        'name': interface.name,
        'wl_name': interface.wl_name,
        'methods': ({'method': generate_method(interface.name, request)}
                    for request in interface.requests),
    }
    return render(STRUCT_TEMPLATE, data)

EMPTY_REQUEST_TEMPLATE = '''
    {{#description}}
    {{line}}
    {{/description}}
    pub fn {{name}}({{formal_arguments}}) {
        {{#is_destructor}}
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        {{/is_destructor}}
        {{^is_destructor}}
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        {{/is_destructor}}
        {{#translations}}
        {{line}}
        {{/translations}}
        unsafe {
            ffi::wayland::wl_proxy_marshal(
                {{call_arguments}}
            );
        }
    }'''

FACTORY_REQUEST_TEMPLATE = '''
    {{#description}}
    {{line}}
    {{/description}}
    pub fn {{name}}({{formal_arguments}}) -> Result<::client::protocol::{{result}}, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        {{#translations}}
        {{line}}
        {{/translations}}
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                {{call_arguments}}
            )
        };
        return ::client::protocol::{{result}}::from_mut_ptr(object);
    }'''

DYNAMIC_REQUEST_TEMPLATE = '''
    {{#description}}
    {{line}}
    {{/description}}
    pub fn {{name}}<T: FromRawPtr<ffi::wayland::WLProxy> + GetInterface>({{formal_arguments}}) -> Result<T, &'static str> {
        let proxy = self.as_mut_ptr() as *mut ffi::wayland::WLProxy;
        {{#translations}}
        {{line}}
        {{/translations}}
        let object = unsafe {
            ffi::wayland::wl_proxy_marshal_constructor(
                {{call_arguments}}
            )
        };
        return T::from_mut_ptr(object);
    }'''

def generate_method(interface_name, request):
    arguments = ['&mut self']
    translations = list()
    access = ['proxy', '{}::{} as u32'.format(interface_name + 'Request', request.name)]

    if isinstance(request, scanner.context.EmptyRequest):
        if request.is_destructor:
            arguments = ['mut self']
    elif isinstance(request, scanner.context.FactoryRequest):
        access.append('::client::protocol::{}::get_interface()'.format(request.interface))
        access.append('ptr::null::<c_void>()')
    elif isinstance(request, scanner.context.DynamicRequest):
        access.append('T::get_interface()')

    types = {'int': 'i32', 'fd': 'RawFd',
            'uint': 'u32', 'fixed': 'i32',
             'object': '&mut ::client::protocol::{}',
            'string': '&str',
            'array': '*mut ffi::wayland::WLArray'}

    char_arguments = list()
    for argument in request.arguments:
        arg = '{}: {}'.format(argument.name, types.get(argument.type))
        acc = argument.name

        if argument.type == 'object':
            arg = arg.format(argument.interface)
            translation = 'let {0}pointer = {0}.as_mut_ptr() as *mut ffi::wayland::WLProxy;'
            translations.append(translation.format(argument.name))
            acc += 'pointer'
        elif argument.type == 'string':
            translation = 'let {0}pointer = CString::new({0}).unwrap().as_ptr();'
            translations.append(translation.format(argument.name))
            acc += 'pointer'

        arguments.append(arg)
        access.append(acc)

    if isinstance(request, scanner.context.DynamicRequest):
        arguments.append('version: u32')
        access.append('(*T::get_interface()).name')
        access.append('version')
        access.append('ptr::null::<c_void>()')

    data = {
        'description': process_description(request.description),
        'is_destructor': isinstance(request, scanner.context.EmptyRequest) and request.is_destructor,
        'name': request.method_name,
        'formal_arguments': ', '.join(arguments),
        'translations': ({'line': line} for line in translations),
        'call_arguments': ', '.join(access),
    }
    if isinstance(request, scanner.context.EmptyRequest):
        return render(EMPTY_REQUEST_TEMPLATE, data)
    elif isinstance(request, scanner.context.FactoryRequest):
        data['result'] = request.interface
        return render(FACTORY_REQUEST_TEMPLATE, data)
    elif isinstance(request, scanner.context.DynamicRequest):
        return render(DYNAMIC_REQUEST_TEMPLATE, data)

EVENT_DISPATCHER_TEMPLATE = '''
#[allow(unused_variables)]
extern fn {{wl_name}}_event_dispatcher<T: {{name}}EventHandler>(
        user_data: *mut c_void,
        _target: *mut c_void,
        opcode: uint32_t,
        _message: *const ffi::wayland::WLMessage,
        arguments: *mut ffi::wayland::WLArgument) -> c_int {
    let object = user_data as *mut T;
    return match {{name}}Event::from_u32(opcode) {
        Some(event) => {
            match event {
            {{#event_matchers}}
                {{name}} => {
                    {{#lines}}
                    {{line}}
                    {{/lines}}
                },
            {{/event_matchers}}
            }
            0
        },
        _ => -1,
    }
}'''

def generate_event_dispatcher(interface):
    if not interface.events: return ''
    TEMPLATE = 'let {} = unsafe {{ *(*arguments.offset({})).{}() }};'
    event_matchers = list()
    for event in interface.events:
        translations = list()
        for (num, arg) in enumerate(event.arguments):
            if arg.type == 'int':
                translations.append(TEMPLATE.format(arg.name, num, 'int'))
            elif arg.type == 'uint':
                translations.append(TEMPLATE.format(arg.name, num, 'uint'))
            elif arg.type == 'new_id':
                translations.append(TEMPLATE.format(arg.name + '_ptr', num, 'new_id'))
                translations.append('let {0} = FromRawPtr::from_mut_ptr({0}_ptr).unwrap();'.format(arg.name))
            elif arg.type == 'fixed':
                translations.append(TEMPLATE.format(arg.name, num, 'fixed_point'))
            elif arg.type == 'string':
                translations.append(TEMPLATE.format(arg.name + '_raw', num, 'string'))
                translations.append('let {0}_buffer = unsafe {{ CStr::from_ptr({0}_raw).to_bytes() }};'.format(arg.name))
                translations.append('let {0} = String::from_utf8({0}_buffer.to_vec()).unwrap();'.format(arg.name))
            elif arg.type == 'object':
                translations.append(TEMPLATE.format(arg.name, num, 'object'))
            elif arg.type == 'array':
                translations.append(TEMPLATE.format(arg.name, num, 'array'))
            elif arg.type == 'fd':
                translations.append(TEMPLATE.format(arg.name, num, 'file_descriptor'))
        arguments = [arg.name for arg in event.arguments]
        translations.append('unsafe {{ (*object).on_{}({}); }}'.format(event.wl_name, ', '.join(arguments)))
        event_matchers.append({
            'name': '{}Event::{}'.format(interface.name, event.name),
            'lines': ({'line': line} for line in translations),
        })
    if len(interface.events) == 1:
        event_matchers.append({
            'name': '_',
            'lines': [],
        })

    data = {
        'wl_name': interface.wl_name,
        'name': interface.name,
        'event_matchers': event_matchers,
    }
    return render(EVENT_DISPATCHER_TEMPLATE, data)

EVENT_HANDLER_TEMPLATE = '''
pub trait {{name}}EventHandler: Sized {
    fn connect_dispatcher(&mut self) {
        unsafe {
            ffi::wayland::wl_proxy_add_dispatcher(
                {{#is_display}}
                self.get_{{wl_name}}().as_mut_ptr() as *mut ffi::wayland::WLProxy,
                {{/is_display}}
                {{^is_display}}
                self.get_{{wl_name}}().as_mut_ptr(),
                {{/is_display}}
                {{wl_name}}_event_dispatcher::<Self>,
                self as *mut Self as *mut c_void,
                ptr::null_mut());
        }
    }

    fn get_{{wl_name}}(&mut self) -> &mut {{name}};
    {{#methods}}
    {{method}}
    {{/methods}}
}'''

def generate_event_handler(interface):
    if not interface.events: return ''
    data = {
        'is_display': interface.wl_name == 'display',
        'name': interface.name,
        'wl_name': interface.wl_name,
        'methods': ({'method': generate_event_handler_method(interface.name, event)}
                    for event in interface.events),
    }
    return render(EVENT_HANDLER_TEMPLATE, data)

EVENT_HANDLER_METHOD_TEMPLATE = '''
    {{#description}}
    {{line}}
    {{/description}}
    #[allow(unused_variables)]
    fn on_{{wl_name}}({{arguments}}) {}'''

def generate_event_handler_method(interface_name, event):
    types = {'int': 'i32', 'fd': 'RawFd',
            'uint': 'u32', 'fixed': 'i32',
            'new_id': '::client::protocol::{}',
             'object': '*mut ffi::wayland::WLObject',
            'string': 'String',
            'array': '*mut ffi::wayland::WLArray'}
    arguments = ['&mut self']
    arguments.extend(arg.name + ': ' + types.get(arg.type, '').format(arg.interface)
                 for arg in event.arguments)
    data = {
        'description': process_description(event.description),
        'wl_name': event.wl_name,
        'arguments': ', '.join(arguments),
    }
    return render(EVENT_HANDLER_METHOD_TEMPLATE, data)
