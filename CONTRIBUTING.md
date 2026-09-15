# Contributing to woodpecker

Hi all, thanks for considering contributing to this little project!

All types of contributions are appreciated! Please see the [Table of Contents](#table-of-contents) for different ways to help out and details about how your contributions will be handled. Please ensure to read the relevant sections before making your contribution, as it will make it a lot easier for me, and smooth out the experience for all involved!

## Table of Contents

* [Code of Conduct](#code-of-conduct)
* [Questions](#questions)
* [I Want to Contribute](#i-want-to-contribute)
* [Reporting Bugs](#reporting-bugs)
* [Suggesting Enhancements](#suggesting-enhancements)
* [Style Guide](#style-guide)

## Code of Conduct

This project and all participating in it are governed by the [Code of Conduct](./CODE-OF-CONDUCT.md). By participating you are expected to uphold this code.

## Questions

Before you ask a question, it is best to search for existing [issues](https://github.com/v0idx/woodpecker/issues) that might help. In case you have found a suitable issue and still need clarification you can write your question in this issue. It is also advisable to do your own research first.

If you still feel it necessary to ask a question, or gain clarification, then I recommend:

* Opening an [issue](https://github.com/v0idx/woodpecker/issues)
* Providing as much context as possible about what you're running into
* Provide project and platform versions (cargo, rustc, etc) depending on what seems relevant

I will then take care of the issue as soon as possible!

## I Want To Contribute

> ### Legal Notice
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

#### Submitting a Bug Report

A good bug report shouldn't require others to follow up for more information. Therefore I ask you to please investigate carefully, collect any relevant information, and describe the bug you're encountering in detail within the report. Please complete the following steps in advance to help ensure the bug-fixing process is completed as fast as possible.

* Ensure that you are using the latest version
* Determing that the bug is really a bug, and not an error on your side e.g. using incompatible environment components/versions
* Check if other users have experienced (and maybe already solved!) the same issue you are having, ensure that there isn't already a report filed on the [issues page](https://github.com/v0idx/woodpecker/issues)
* Search the internet (including resources such as StackOverflow) to see if other users have encountered and discussed the issue
* Collect information about the bug
* Collect a Stack Trace
* Collect OS, Platform, and Version
* Collect your version of cargo, rustc, and packages used
* Collect your input and output if relevant
* Is the bug reliably reproducible

##### How To Submit a Good Bug Report

I use GitHub issues to track bugs and errors. If you encounter an issue with the project:

* Open an [issue](https://github.com/v0idx/woodpecker/issues/new)
* Explain the expected behaviour, and the actual behaviour encountered
* Provide as much context as possible, and describe the reproduction steps that someone else can follow to recreate the issue on their own. This usually includes your code. To help me out more please isolate the problem and produce a reduced test case
* Provide the information collected in the [previous section](#submitting-a-bug-report)

Once it's filed:

* I will label the issue accordingly
* I will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, I may ask you for these steps and mark the issue as `needs-repro`. Bugs marked as `needs-repro` will not be addressed until they are reproduced
* If I'm able to reproduce the issue, it will be marked `needs-fix`, as well as possible other tags, and the issue will be left to be implemented by someone

#### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for woodpecker, including new features and improvements to existing functionality.

##### Before Submitting an Enhancement

* Make sure that you are using the latest version
* Ensure that the functionality you are suggesting is not already covered, perhaps by a flag
* Perform a [search](https://github.com/v0idx/woodpecker/issues) to ensure that the enhancement has not already been suggested. If it has, please add a comment to the existing issue instead of opening a new one!
* Find out whether your idea fits the scope and aims of the project, it's up to you to make a strong case for the inclusion of the suggestion. Keep in mind that features that will be useful to the majority of users and not just a small sub-set.

##### How Do I Submit a Good Enhancement Suggestion?

As with bugs and errors, GitHub issues are used to track enhancement suggestions

* Please use a clear and descriptive title for the issue to identify the suggestion
* Provide a step-by-step description of the suggested enchancement, in as much detail as possible
* Describe the current behaviour and explain what behaviour you expect to see instead, and why
* You may want to include screenshots or animated GIFs which help to demonstrate the steps or point out the areas in which the suggestion is related to.
* Explain why the enhancement would be useful to most users. You may also want to point out other projects that have solved similar issues, and that could serve as inspiration.

### Style Guide

#### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull-requests liberally after the first line

#### Rust Code

* Always run the prek pre-commit checks on any Rust code committed
* Use human legible variable and method names
* Avoid the usage of single-character variable names
* Ensure code is well commented
* Reference external repositories when inspiration or methods are used from them
* When writing tests, name the test functions as such `test_function_name()`

#### Markdown

* Always run the prek pre-commit checks on any Markdown committed

### Attribution

This guide is based upon the example given at [contributing.md](https://contributing.md/example)
