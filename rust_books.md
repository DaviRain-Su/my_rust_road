# Rust Book 目录

# Rust In Action

> T.S McMamara

## Introducing Rust

- How is Rust used?
- What is it like to advocate for Rust at work?
- What does Rust look and feel like?
- What is Rust?
  - Safety
  - Productivity
  - Control
- Rust's Big Features
  - Performance
  - Concurrency
  - Memory Efficiency
- Downsides of Rust
  - Cyclic data structures
  - Compile times
  - Strictness
  - Size of the Language
  - Hype
- Where does Rust fit best?
  - Command-line utilities
  - Data Processing
  - Extending an Application
  - Resource-constrained environments, such as microcontrollers
  - Server-size applications
  - Desktop applications
  - Desktop
  - Mobile
  - Web
  - System Programming
- Rust's hidden featuer: Its community
- A Taste of the language
  - Cheating Your Way To  "Hello, World!"
  - Your First Rust Program
- Rust phrase book
- Summary

# Rust Language Distinctives

## Language Foundations

- A Glance at Rust's Syntax
  - A whole program with main()
  - Start out with Numbers
  - Type-aware control flow with match
  - Getting Stuff Done with functions
  - Creating grep-lite v1
  - Make Lists of Things with Arrays, Slices and Vectors
  - Including Third Party Code
  - Supporting Command Line Arguments
  - Reading from Files
  - Reading from STDIN
- Summary

## Compound Data Typs

- Using plain functions to experiment with an API
- Modeling Files With struct 
- Adding Methods to a struct with  impl
  - Simplifying object creating by implementing a new() method
- Returning errors
  - Modifying a known global variable
  - Making use of the Result return type
- Defining and marking use of enum
  - Using an enum to manage internal state
- Defining Common Behavior with Traits
  - Creating a Read trait
  - Implementing Display for your own types
- Exposing your types to the world
  - Protecting private data
- Creating in-line Documentation
  - Using rustdoc to Render Docs For a Single Source File
  - Using Cargo to Render Docs for Crate and its Dependencies
- Summary





## Lifetimes, Ownership and Borrowing

- "Implementing" a Mock CubeSat Ground Station
  - Encountering our first lifetime issue
  - Special behavior of primitive types
- Guide to the figures in this chapter
- How Ownership Moves
- Resolving Ownership Issues
  - Use references where full ownership is not required
  - Use Fewer Long-Lived Values
  - Duplicate the value
  - Wrap Data Within Specialty Types
- Summary



# Demystifying System Programming

## Data in Depth

- Bit Patterns and Types
- Life of an integer
  - Understanding Endianness
- Decimal Numbers
  - About Floating Point
  - Looking inside an f32
  - About the Sign Bit
  - About the Exponent
  - About the Mantissa
  - Representing decimal numbers in a single bytes a fixed-point number format
- Generating f32 values between 0 and 1 from random bytes
- Implementing a CPU in Software to Establish that Functions are also Data
  - CPU 1: "the Adder"
  - First working emulator
  - CPU 2: "the Multi-Adder"
  - CPU 3:  Adding functions
  - CPU 4: Adding the rest
- Summary

## Memory

## Files and Storage

## Networking

## Time and Time Keeping

## Process, Threads and Containers

## Kernel

## Signal,Interrupts, and Exceptions



# Rust Programming By Example

> Enter the world of Rust by building engaging, concurrent, reactive, and roburst applications

## Basics of Rust

## Starting with SDL 

## Events and Basic Game Mechanisms

## Adding All Game Mechanisms

## Creating a Music Player

## Implementing the Engine of the Music Player

## Music Player in More Rusty Way with Relm

## Understanding FTP

## Implementing an Asynchronous FTP Server

## Implementing Asynchronous File Transfer

## Rust Best Practices



# Rust Essentials

> A quick guide to writing fast, safe, concurrent, systems and applications

## Starting with Rust

## Using Variables and Types

## Using Functions anf Control Structures

## Structuring Data and Matching Patterns

## Higher Order Functions and Erro-Handing

## Using Traits and OOP in Rust

## Ensuring Memory Safety and Pointers

## Organizing Code and Macros

## Concurrency - Code for Multicore Execution

## Programming at the Boundaries

## Exploring the Standard Library

## The Ecosystem of Crates

