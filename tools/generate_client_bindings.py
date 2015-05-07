# Copyright (c) <2015> <lummax>
# Licensed under MIT (http://opensource.org/licenses/MIT)

import os.path
import scanner

def main():
    ast = scanner.build_ast("wayland_1.7.0.xml")
    generator = scanner.generate_client_protocol_bindings(ast)
    for (filename, content) in generator:
        path = os.path.join("src/client/protocol", filename)
        with open(path, 'w') as fileob:
            print(content, file=fileob)

if __name__ == "__main__": main()
