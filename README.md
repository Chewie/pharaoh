# Pharaoh : build that test pyramid!

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/chewie/pharaoh/ci)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/chewie/pharaoh)
![Codecov](https://img.shields.io/codecov/c/github/chewie/pharaoh)

## What it is

Pharaoh is a dead simple, no permission needed, functional test runner for
command line applications, written in Rust.

This tool scans for YAML entries in a folder (by default, the current
directory) and run all tests in it.

An example YAML is as follow:

~~~yaml
name: this test will succeed
cmd: echo foo
stdout: |
  foo
---
name: this test will fail
cmd: echo fou
stdout: |
  foo
---
name: cat should work
cmd: cat
stdin: |
  this is a line
stdout: |
  this is a line
---
name: failing for all three reasons
cmd: cat
stdin: |
  my input
stdout: |
  a different output
stderr: |
  an unexpected stderr output
status: 1
~~~

For each entry in such a file, Pharaoh will run the given command, feed it the
specified stdin, and compare the captured output to the specified stdout,
stderr and exit code. If not specified, these values default to respectively
empty strings and 0.

## Made in TDD, for TDD

Most often, what you will want to run is the program you are working on, and
run functional tests over it. Specifically, Pharaoh is made to practice double
loop TDD, and should be complemented with unit tests for your project-specific
language.

For example, the first few tests for a calculator could be as follows:

~~~yaml
name: do nothing if stdin is empty
cmd: ./myevalexpr
---
name: evaluate a single expression
cmd: ./myevalexpr
stdin: |
  2 + 4
stdout: |
  6
---
name: evaluate multiple lines
cmd: ./myevalexpr
stdin: |
  5 + 2
  8 * 5
stdout: |
  7
  40
---
name: handle syntax errors
cmd: ./myevalexpr
stdin: |
  2 + +
stderr: |
  "2 + +": syntax error
status: 1
~~~

## Who is this project for?

Pharaoh was designed with school projects and katas in mind, or more generally
any kind of short-lived project where building your own tool is not worth it.

It is generally difficult to teach TDD to students, as their project are
usually weeks long at most, and no testing tools are provided. Requiring
students to develop their own tool in addition to their project is by
experience too high of a step, and they default to testing by hand.

In addition, many students use restricted environment with limited permissions,
and as such cannot use fancy tools that may use container technologies like
docker. Pharaoh is a drop-in static binary that requires no further
installation or configuration, and as such is ideal for those use cases.

## A guided example: developing an application in double-loop TDD

Coming soon!
