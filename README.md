# gitignore

**Build _.gitignore_ files with Alfred.**

This is an [Alfred](https://www.alfredapp.com) workflow that lets you create
_.gitignore_ files from Alfred's input interface. It uses templates
provided by [GitHub](https://github.com), and combines them into a single
.gitignore file.

## Requirements

Although it should be pretty self-explanatory, these are the requirements for
this workflow:

- **OS X**
- **Alfred 3**
- **Git**

## Installation

After installing the workflow, you need to download the templates. Enter the
following command into Alfred:

```
gitignore-update
```

Executing this will clone the [github/gitignore](https://github.com/github/gitignore)
repository, and make the templates in it available to you.

## Usage

To use this workflow, simply type in `gitignore`. You will now see a list of all
templates installed on your machine. You can search for specific templates by
typing in their name. Selecting a template will place add it to the command
line.

If you've selected all templates that you want to combine, simply select the
first item in the list called _Build .gitignore file_. This will start the
generation of the template, and open it in TextEdit once it has been created.

Copy & paste the contents of the file and paste them into the `.gitignore` file
in your project.

The temporary file is created in `/tmp/`, and will automatically be deleted
after three days.

## License

This project is open source under the terms of the MIT license. See
[LICENSE.txt](./LICENSE.txt) for more details.
