# encoding: utf-8

import os
import re
import sys
from sh import git, pwd, sh
from workflow import Workflow, ICON_SYNC, web

workflow = Workflow()
repo_dir = workflow.datafile("gitignore")


def main(wf):
    if not os.path.isdir(repo_dir):
        clone_repo()
    else:
        pull_repo()

    update_templates()

    print "Templates have been successfully updated."


def clone_repo():
    try:
        os.chdir(workflow.datafile(""))
        git.clone("https://github.com/github/gitignore.git")
    except:
        handle_exception()
    return 0


def pull_repo():
    try:
        os.chdir(repo_dir)
        git.pull()
    except:
        handle_exception()
    return 0


def handle_exception():
    e = sys.exc_info()[0]
    if e.__name__ == "ErrorReturnCode_128":
        print "'git clone' failed due to an unknown reason. Please contact the support."
    else:
        print "An unknown error occured. Please contact the support."
    sys.exit(-1)


def update_templates():
    workflow.clear_data(lambda f: f.startswith("templates"))
    store_template_names()
    return 0


def store_template_names():
    templates = get_template_names()
    templates.sort()

    workflow.store_data('templates', templates)
    return 0


def get_template_names():
    file_names = get_file_names_in_dir(repo_dir)
    templates = []

    for f in file_names:
        file_name = str(f)
        if re.search(".\.gitignore$", file_name):
            templates.append(file_name[:-10])

    return templates


def get_file_names_in_dir(directory):
    file_names = []

    for root, subdirs, files in os.walk(directory):
        for subdir in subdirs:
            file_names.append(get_file_names_in_dir(subdir))

        for f in files:
            file_names.append(f)

    return file_names


if __name__ == u"__main__":
    sys.exit(workflow.run(main))
