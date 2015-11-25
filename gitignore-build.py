# encoding: utf-8

import glob
import hashlib
import os
import sys
from workflow import Workflow, ICON_SYNC, ICON_WARNING, web

workflow = Workflow()
repo_dir = workflow.datafile("gitignore")

def main(wf):
    if len(sys.argv) < 2:
        print "No templates were selected, so nothing was built."
        return

    if not os.path.isdir(repo_dir):
        print "Please run gitignore-update first to download the templates."

    templates = sys.argv[1:]

    tmp_file_name = hashlib.md5(" ".join(templates)).hexdigest()
    tmp_file_path = "/tmp/" + tmp_file_name

    if os.path.isfile(tmp_file_path):
        os.system("open %s" % tmp_file_path)
        return

    formatted_templates = set()

    for t in templates:
        formatted_templates.add(t.lower() + ".gitignore")

    for root, dirs, files in os.walk(repo_dir):
        for name in files:
            if name.lower() in formatted_templates:
                with open(os.path.join(root, name)) as in_file:
                    with open(tmp_file_path, "a+") as out_file:
                        out_file.write("### %s\n\n" % name)
                        for line in in_file:
                            out_file.write(line)
                        out_file.write("\n\n")

    print "Successfully built .gitignore file. Have fun!"
    os.system("open %s" % tmp_file_path)
    return


if __name__ == u"__main__":
    sys.exit(workflow.run(main))
