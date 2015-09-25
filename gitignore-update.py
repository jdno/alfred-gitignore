# encoding: utf-8

import os
import sys
from sh import git, pwd, sh
from workflow import Workflow, ICON_SYNC, web


def main(wf):
    return_value = 0

    if not repo_exists():
        return_value = clone_repo()
    else:
        return_value = pull_repo()

    if return_value:
        print "ERROR. Templates could not be downloaded."
    else:
        print "Templates have been successfully updated."


def clone_repo():
    return_value = 0

    try:
        return_value = git.clone("https://github.com/github/gitignore.git")
    except:
        return_value = -1

    return return_value


def pull_repo():
    return_value = 0

    try:
        os.chdir("./gitignore")
        return_code = git.pull()
    except:
        return_value = -1

    return return_value


def repo_exists():
    return os.path.isdir("./gitignore")


if __name__ == u"__main__":
    wf = Workflow()
    sys.exit(wf.run(main))
