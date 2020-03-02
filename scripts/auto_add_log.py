#!/user/bin/python3

import sys
import re
import subprocess
import os
'''
automatically add progress log to README.md
'''

from datetime import datetime
x = datetime.now()
formated_date = '/'.join([str(i) for i in [x.day, x.month, x.year]])


def parse_problem(pro_file):
        # extract problem [link, difficulty, id]
    con = open(pro_file, 'r').read()
    plink = re.search(r'(https://.*)', con, re.M | re.I).group(1)
    diffi = re.search(r'# (Medium|Easy|Hard)', con, re.M | re.I)
    diffi = diffi.group(1) if diffi else 'Easy'
    pid = pro_file.replace('/', '.').split(".")[1]
    title = pro_file.replace('/', '.').split(".")[2]
    return [plink, diffi, pid, title]


def format_print():
    listfiles = subprocess.Popen(["ls", "-lt", "solutions"], stdout=subprocess.PIPE).stdout
    outputs = []

    for fs in listfiles:
        # print(fs)
        for line in str(fs).split("\n"):
            arr = line.split()
            rm = re.match(r'\d+.*\.rs', arr[-1])
            if rm is not None:
                local_link = "solutions/" + rm.group(0)
                plink, diffi, pid, title = parse_problem(local_link)
                outputs.append([pid, diffi, plink, title, "./" + local_link])

    outputs.sort(key=lambda a: int(a[0]))
    return outputs

def find_rust_solution(local):
    return local if os.path.exists(local) else None

def update_readme():
    diff_symbols = {'Hard': '𝐇', 'Medium': '𝐌', 'Easy': '𝐄'}
    readme_head = open('conf.d/readme_head', 'r').read()
    items = format_print()
    with open('README.md', 'w') as f:
        f.write(readme_head)
        for pid, diffi, plink, title, local in items:
            tr = "|" + diff_symbols[diffi] + "." + pid + " | [" + title + "](" + plink + ") | [Rust solution](" + local + ")"
            tr += "|\n"
            f.write(tr)
        f.write(open('conf.d/readme_tail', 'r').read())

update_readme()

