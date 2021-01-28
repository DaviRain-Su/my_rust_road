# Crate mio 

Mio 是一个针对Rust的快速、低级I/O库，专注于非阻塞api和事件通知，用于构建高性能I/O应用程序，并尽可能减少操作系统抽象的开销。 

## Usage 

使用 Mio 首先要创建一个Poll, 它从操作系统读取事件并将他们放入Events中。 
你可以使用它来处理来自操作系统的I/O事件。

## Example 

例子可以在源代码的示例目录或者github上找到

## Guide 

在指南模块中有一个入门指南

## Available features 可用特性

特性模块中描述了可用的特性


## Mudules 

event Readiness event types and unilities 
features Mio's optonal features 
guide Getting started guide 
net tcp/udp/uds Networking primitives 
unix Unix/os-util Unix only extensions 

## Structs 

Interest Interest used in registering 
poll Polls for readiness events on all registered values 
Registty Resisters I/O resources
Token Associates readiness events with event::sources 
Waker Waker allows cross-thread waking of Poll 