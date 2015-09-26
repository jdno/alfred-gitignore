# encoding: utf-8

import os
import re
import sys
from sh import git, pwd, sh
from workflow import Workflow, ICON_SYNC, web

workflow = Workflow()
repo_dir = workflow.datafile("gitignore")


def main(wf):
    return_value = 0

    if not os.path.isdir(repo_dir):
        return_value = clone_repo(wf.datafile(""))
    else:
        return_value = pull_repo(repo_dir)

    update_templates(repo_dir)

    if return_value:
        print "ERROR. Templates could not be downloaded."
    else:
        print "Templates have been successfully updated."


def clone_repo(parent_dir):
    return_value = 0

    try:
        os.chdir(parent_dir)
        return_value = git.clone("https://github.com/github/gitignore.git")
    except:
        return_value = -1

    return return_value


def pull_repo():
    return_value = 0

    try:
        os.chdir(repo_dir)
        return_code = git.pull()
    except:
        return_value = -1

    return return_value


def update_templates():
    wf.clear_data(lambda f: f.startswith("templates"))
    store_template_names(repo_dir)


def store_template_names():
    templates = get_template_names(repo_dir)
    templates.sort()
    wf.store_data('templates', templates)


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
