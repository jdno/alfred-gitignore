# encoding: utf-8

import os
import re
import sys
from sh import git, pwd, sh
from workflow import Workflow, ICON_SYNC, web

workflow = Workflow()
repo_dir = workflow.datafile("gitignore")


def main(wf):
    """
    Run script.

    This script checks whether or not the gitignore repository has already been
    cloned, and either clones it or pulls the latest changes from the remote
    repository. In both cases, the list of templates is stored in the persistent
    storage provided by the Workflow's data API.
    """
    if not os.path.isdir(repo_dir):
        clone_repo()
    else:
        pull_repo()

    update_templates()

    print "Templates have been successfully updated."


def clone_repo():
    """
    Clone the Git repository 'github/gitignore' to the data directory.

    This function clones the gitignore repository from GitHub and saves the
    local copy in the workflow's data directory. It uses the module sh to invoke
    the git executable. If the git executable cannot execute properly, an
    exception is thrown.
    """
    try:
        os.chdir(workflow.datafile(""))
        git.clone("https://github.com/github/gitignore.git")
    except:
        handle_exception()
    return 0


def pull_repo():
    """
    Pull the recent changes from origin master.

    This function pulls all recent changes from the gitignore repository on
    GitHub. It uses the module sh to invoke the git executable. If the git
    executable cannot execute properly, an exception is thrown.
    """
    try:
        os.chdir(repo_dir)
        git.pull()
    except:
        handle_exception()
    return 0


def handle_exception():
    """
    Handle the last thrown exception.

    This function handles the last thrown exception. It compares the exception's
    class to a number of known exceptions, and prints the respective error
    message. If the exception is not known, a generic error message is printed.
    """
    e = sys.exc_info()[0]
    if e.__name__ == "ErrorReturnCode_128":
        print "'git clone' failed due to an unknown reason. Please contact the support."
    else:
        print "An unknown error occured. Please contact the support."
    sys.exit(-1)


def update_templates():
    """
    Update the list of templates stored with the Workflow's data API.

    This function updates the list of templates that is stored with the
    Workflow's data API. To avoid duplicate data entries, it first deletes any
    existing data before saving the current list of templates.
    """
    workflow.clear_data(lambda f: f.startswith("templates"))
    store_template_names()
    return 0


def store_template_names():
    """
    Save the template names using the Workflow's data API.

    This function reads the names of the currently available templates from the
    directory, and saves them in the persistent data storage provided by the
    Workflow library.
    """
    templates = get_template_names()
    templates.sort()

    workflow.store_data('templates', templates)
    return 0


def get_template_names():
    """
    Return the names of all templates in the local repository.

    This function goes recursively through the local copy of the gitignore
    repository, and returns the name of all templates within it. Templates are
    identified by their file extension, which is '.gitignore'.
    """
    file_names = get_file_names_in_dir(repo_dir)
    templates = []

    for f in file_names:
        file_name = str(f)
        if re.search(".\.gitignore$", file_name):
            templates.append(file_name[:-10])

    return templates


def get_file_names_in_dir(directory):
    """
    Return the names of all files in the given directory.

    Arguments:
    - directory: Path of the directory whose files should be returned

    This function goes recursively through the given directory and returns the
    name of all files within it.
    """
    file_names = []

    for root, subdirs, files in os.walk(directory):
        for subdir in subdirs:
            file_names.append(get_file_names_in_dir(subdir))

        for f in files:
            file_names.append(f)

    return file_names


if __name__ == u"__main__":
    sys.exit(workflow.run(main))
