#!/usr/bin/env python3
# Copyright (c) 2019 Xu Shaohua <xushaohua2016@outlook.com>. All rights reserved.
# Use of this source is governed by Apache License that can be found
# in the LICENSE file.

import os
import re
import subprocess
import sys
import urllib.request


def download_syscall_file(url):
    with urllib.request.urlopen(url) as f:
        return f.read()


def parse_sysno(content):
    sysnum_pattern = re.compile("^([0-9]+)\s*\S*\s*STD\s*({\s*\S*\s*(\w+).*)$")
    proto_pattern = re.compile("\s{2,}")

    # Merge multiple lines into one
    content = content.replace("\\\n", "")

    lines = [
        "",
        "// Code generated by mksysnum_freebsd.py; DO NOT EDIT.",
        "",
        "pub type Sysno = usize;",
        "",
    ]
    names = []

    for line in content.split("\n"):
        m = sysnum_pattern.match(line)
        if m:
            num = m.group(1)
            proto = m.group(2)
            name = m.group(3)

            if name == "sys_exit":
                name = "exit"

            # Replace multiple space chars with only one
            proto = proto_pattern.sub(" ", proto)

            line = "pub const SYS_{0}: Sysno = {1}; // {2}".format(name.upper(), num, proto)
            names.append(name)
            lines.append(line)
    return (lines, names)


def print_sysnums(names):
    temp = '''
pub fn %s() {
    core::unimplemented!();
    // syscall0(SYS_%s);
}
'''
    raw_identifiers = ["break", "yield"]
    output = []
    for name in names:
        upper_name = name.upper()
        if name in raw_identifiers:
            name = "r#" + name
        output.append(temp % (name, upper_name))
    return output


def rust_fmt(filename):
    subprocess.run(["rustfmt", filename])


def main():
    if len(sys.argv) != 2:
        print("Usage: %s os arch" % sys.argv[0])
        sys.exit(1)

    arch_name = sys.argv[1]
    platform_folder = os.path.join("platform", "freebsd-%s" % arch_name)

    url = "https://svn.freebsd.org/base/stable/12/sys/kern/syscalls.master"
    content = download_syscall_file(url)

    lines, names = parse_sysno(content.decode())
    sysnum_file = os.path.join(platform_folder, "sysno.rs")
    with open(sysnum_file, "w") as fh:
        fh.write("\n".join(lines))
    rust_fmt(sysnum_file)

    call_file = os.path.join(platform_folder, "call.rs")
    with open(call_file, "w") as fh:
        fh.writelines(print_sysnums(names))
    rust_fmt(call_file)


if __name__ == "__main__":
    main()
