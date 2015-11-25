# gitignore
## Build _.gitignore_ files with Alfred.

**gitignore** is an [Alfred](https://www.alfredapp.com) workflow that lets you
create _.gitignore_ files from Alfred's input interface. It uses templates
provided by [GitHub](https://github.com), and combines them into a single
.gitignore file.

## Requirements

Although it should be pretty self-explanatory, these are the requirements for
this workflow:

- **OS X**
- **Alfred 2**
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
first item in the list called "Build .gitignore file". This will start the
generation of the template, and open it in TextEdit once it has been created.

Copy & paste the contents of the file and paste them into the `.gitignore` file
in your project.

The temporary file is created in `/tmp/`, and will automatically be deleted
after three days.

## License

Copyright (c) 2015 Jan David Nose

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
