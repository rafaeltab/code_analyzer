# Glossary

This document describes some commonly used terms

## Module
A loadable piece of code that can be developed by external developers.

## Analyzer
A piece of code that takes an abstract syntax tree, and analyzes it in some way.
The analysis result are diagnostics, and code actions.

## Parser
A piece of code that takes in a buffer and parses it for a specific language.
The output is an abstract syntax tree.

## Project
Always refers to the `code_analyzer` project.

## Package
A reusable piece of code.
It can be compiled into a library that can be used by other code.

## App
A piece of code that can be used by an end user. Examples are an RPC server, CLI, or LSP server.
It can be compiled into a binary that can be executed.

## Workspace
Either a package or an app.
It can be compiled into either an executable binary or library.

## Diagnostic
A message associated with a position and severity level that indicates a (possible) problem in a buffer.

## Code action
A change that can be applied to a buffer along with a position.

## Abstract Syntax Tree (AST)
A tree representation of the source code of a computer program that conveys the structure of the source code.
