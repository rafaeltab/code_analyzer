# Code analyzer

A single system for parsing, linting, and analyzing code.

## Core values

### Modular
It is crucial that the system is as modular as possible. Creating a modular system has several advantages, the most prominent being performance, and ease of integration. A modular system allows external developers to integrate with the system.

### Documented
Implementing a good, well documented, interface for the system further enhances the ease of integration.

### Polyglot
Allowing the system to work across many programming languages is vital to the success of the project. Integration with other languages makes the implementation of parsers easier in the early stages by enabling integration with existing parsers.

### Fast
Performance is a limiting factor for many existing systems, which is detrimental to their usefulness. Therefore, the use of a performant language, and the use of concurrency is integral. Possibly, the use of caching, and other ways of limiting the amount of work actually performed should be pursued.

### Versatile
Exposing several ways of using the system is crucial to it's utility, and effectiveness. Some planned interfaces are an LSP, library, and RPC server.

## Example use cases

### Markdown linting
Parsing markdown files, and markdown comments in code, and providing linting for both under the same interface.

### Prose analysis
Parsing readable text from many code formats and applying prose analysis across these languages.

### Customizable linting
Building custom code analysis specific to projects that represent business, architectural or code quality requirements.
