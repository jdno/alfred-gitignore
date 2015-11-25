# encoding: utf-8

import argparse
import os
import sys
from workflow import Workflow, ICON_SYNC, ICON_WARNING, web

workflow = Workflow()


def main(wf):
    parser = argparse.ArgumentParser()
    parser.add_argument('query', nargs='*', default=None)

    args = parser.parse_args(wf.args)
    templates = wf.stored_data("templates")

    current_query = ""

    if len is None:
        wf.add_item(
            title="Templates missing",
            subtitle="Please run gitignore-update to download the templates...",
            icon=ICON_WARNING,
            valid=False
        )
    else:
        if args.query:
            query = args.query
            input = query[-1]
            current_query = " ".join(query[:-1])
            filtered_templates = [i for i in templates if input.lower() in i.lower()]

            if len(filtered_templates) >= 1:
                templates = filtered_templates

            wf.add_item(
                title="Build .gitignore file",
                subtitle="Combine the chosen templates to a single .gitignore file...",
                uid="build_gitignore",
                arg=" ".join(query),
                valid=True,
            )

            for i in templates:
                add_template(i, query=current_query)
        else:
            for i in templates:
                add_template(i)

    wf.send_feedback()


def add_template(template_name, query=""):
    """
    Add template to output.

    This function adds the given template as a new item to the XML output.
    """
    autocomplete = build_autocomplete(template_name, query)

    workflow.add_item(
        title=template_name,
        uid=template_name,
        autocomplete=autocomplete,
        valid=False
    )


def build_autocomplete(template_name, query):
    """
    Build the autocomplete string.

    From the template name and the current query a new string is built that can
    be used as the value for an item's autocomplete attribute.
    """
    autocomplete = ""

    if len(query) > 0:
        autocomplete = " ".join([query, template_name])
    else:
        autocomplete = template_name

    return " ".join([autocomplete, ""])


if __name__ == u"__main__":
    sys.exit(workflow.run(main))
