# MapReduce: Simplified data processing on large cluster 



## Abstract



MapReduce is a programming model and an associated implementation for processing and generating large datasets that is amenable to a broad variety of real-world tasks. 

Users specify the computation in terms of a map and a reduce function, and the underlying runtime system automatically parallelizes the computation across large-scale clusters of machines, handles machine failures, and schedules intermachine communication to make efficient use of the network and disks.

Programmers find the system  easy to use: more than ten thousand distinct MapReduce programs have been implemented internally at Google over the past four years, and an average of one hundred thousand MapReduce jobs are executed on Google’s clusters every day, processing a total of more than twenty petabytes of data per day.

mapReduce是一个编程模型和相关的实现，用于处理和生成大型数据集，它适用于各种现实世界的任务。

用户用map和reduce函数来指定计算，底层的运行时系统会自动在大规模的机器集群中并行计算，处理机器故障，并安排机器间的通信，以有效利用网络和磁盘。

程序员们发现这个系统很容易使用：在过去的四年里，谷歌内部已经实现了一万多个不同的MapReduce程序，平均每天有十万个MapReduce作业在谷歌的集群上执行，每天处理的数据总量超过二十PB。

---



## 1 Introduction

Prior to our development of MapReduce, the authors and many others at Google implemented hundreds of special-purpose computations that process large amounts of raw data, such as crawled documents, Web request logs, etc., to compute various kinds of derived data, such as inverted indices, various representations of the graph structure of Web documents, summaries of the number of pages crawled per host, and the set of most frequent queries in a given day. 

Most such computations are conceptually straightforward. However, the input data is usually large and the computations have to be distributed across hundreds or thousands of machines in order to finish in a reasonable amount of time.

The issues of how to parallelize the computation, distribute the data, and handle failures conspire to obscure the original simple computation with large amounts of complex code to deal with these issues.

在我们开发MapReduce之前，作者和Google的其他许多人实现了数百种特殊用途的计算，处理大量的原始数据，如抓取的文档、Web请求日志等。来计算各种派生数据，如倒置索引、Web文档的图结构的各种表示方法、每个主机抓取的网页数量汇总、以及某一天最频繁查询的集合。

大多数这样的计算在概念上是简单的。然而，输入的数据通常很大，计算必须分布在数百或数千台机器上，才能在合理的时间内完成。

如何并行化计算、分布数据的问题。和处理故障共谋，用大量复杂的代码来处理这些问题，掩盖了原本简单的计算。

---



As a reaction to this complexity, we designed a new abstraction that allows us to express the simple computations we were trying to perform but hides the messy details of parallelization, fault tolerance, data distribution and load balancing in a library. 

Our abstraction is inspired by the map and reduce primitives present in Lisp and many other functional languages. 

We realized that most of our computations involved applying a map operation to each logical record’ in our input in order to compute a set of intermediate key/value pairs, and then applying a reduce operation to all the values that shared the same key in order to combine the derived data appropriately. 

Our use of a functional model with user-specified map and reduce operations allows us to parallelize large computations easily and to use reexecution as the primary mechanism for fault tolerance.

作为对这种复杂性的反应,我们设计了一个新的抽象，允许我们表达我们试图执行的简单计算,但在一个库中隐藏了并行化、容错、数据分配和负载平衡的混乱细节。

我们的抽象是受到Lisp和许多其他函数式语言中的map和reduce基元的启发。

我们意识到，我们的大部分计算涉及到对输入中的每条逻辑记录应用map操作，以计算一组中间键/值对，然后对所有共享相同键的值应用reduce操作，以适当地组合衍生数据。

我们使用的功能模型与用户指定的map和reduce操作，使我们能够轻松地并行处理大型计算。并以重新执行作为容错的主要机制。

---

The major contributions of this work are a simple and powerful interface that enables automatic parallelization and distribution of large-scale computations, combined with an implementation of this interface that achieves high performance on large clusters of commodity PCs. 

The programming model can also be used to parallelize computations across multiple cores of the same machine.

这项工作的主要贡献是提供了一个简单而强大的接口，可以实现大规模计算的自动并行化和分发，结合这个接口的实现，可以在大型商品PC集群上实现高性能。

该编程模型也可用于并行化,在同一台机器的多个核心上进行计算。

---

Section 2 describes the basic programming model and gives several examples.

In Section 3, we describe an implementation of the MapReduce interface tailored towards our cluster-based computing environment.

Section 4 describes several refinements of the programming model that we have found useful. 

Section 5 has performance measurements of our implementation for a variety of tasks. 

In Section 6, we explore the use of MapReduce within Google including our experiences in using it as the basis for a rewrite of our production indexing system. 

Section 7 discusses related and future work.

第2节描述了基本的编程模型，并给出了几个例子。

在第3节中，我们描述了针对我们基于集群的计算环境而定制的MapReduce接口的实现。

第4节描述了几个我们认为有用的编程模型的改进。

第5节有我们对各种任务的实现的性能测量。

在第6节中，我们探讨了MapReduce在Google中的使用，包括我们使用它作为重写生产索引系统的基础的经验。

第7节讨论了相关和未来的工作。

## 2 Programming Model

The computation takes a set of input key/value pairs, and produces a set of output key/value pairs. 

The user of the MapReduce library expresses the computation as two functions: map and reduce.

计算需要一组输入键/值对，并产生一组输出键/值对。

MapReduce库的用户将计算表达为两个函数：map和reduce。

---

Map, written by the user, takes an input pair and produces a set of intermediate key/value pairs.  

The MapReduce library groups together all intermediate values associated with the same intermediate key and passes them to the reduce function.

 Map，由用户编写，取一个输入对，产生一组中间键/值对。

MapReduce库将所有与同一个中间键相关联的中间值分组，并将它们传递给reduce函数。

---

The reduce function, also written by the user, accepts an intermediate key and a set of values for that key.

It merges these values together to form a possibly smaller set of values. 

Typically just zero or one output value is produced per reduce invocation. 

The intermediate values are supplied to the user’s reduce function via an iterator. 

This allows us to handle lists of values that are too large to fit in memory.

reduce函数也是由用户编写的，它接受一个中间键和该键的一组值。

它将这些值合并在一起，形成一个可能较小的值集。

通常情况下，每次reduce调用只产生零或一个输出值。

中间值通过迭代器提供给用户的 reduce 函数。

这使得我们可以处理那些太大而无法放入内存的值列表。

---

### 2.1 Example

Consider the problem of counting the number of occurrences of each word in a large collection of documents. 

The user would write code similar to the following pseudocode.

考虑计算一个大型文档集合中每个单词的出现次数的问题。

用户将编写类似于以下伪代码的代码。

---

```c
map(String key, String value):
  // key: document name
  // value: document contents
  for each word w in value:
  	EmitIntermediate(w, “1”);

reduce(String key, Iterator values):
  // key: a word
  // values: a list of counts
  int result = 0;
  for each v in values:
		result += ParseInt(v);
	Emit(AsString(result));
```

The map function emits each word plus an associated count of occurrences (just 1 in this simple example). 

The reduce function sums together all counts emitted for a particular word.

map 函数发出每个词加上相关的出现次数（在这个简单的例子中只有 1 次）。

reduce函数将某一特定单词的所有计数相加。

---

In addition, the user writes code to fill in a mapreduce specification object with the names of the input and output files and optional tuning parameters. 

The user then invokes the MapReduce function, passing it to the specification object. The user’s code is linked together with the MapReduce library (implemented in C++). 

Our original MapReduce paper contains the full program text for this example [8]

此外，用户编写代码，在mapreduce规范对象中填写输入和输出文件的名称以及可选的调整参数。

然后用户调用MapReduce函数，将其传递给规范对象。用户的代码与MapReduce库（用C++实现）链接在一起。

我们最初的MapReduce论文包含了这个例子的完整程序文本[8] 

---

More than ten thousand distinct programs have been implemented using MapReduce at Google, including algorithms for large-scale graph processing, text processing, data mining, machine learning, statistical machine translation, and many other areas. 

More discussion of specific applications of MapReduce can be found elsewhere [8, 16, 7].

在Google使用MapReduce实现了一万多个不同的程序，包括大规模图处理、文本处理、数据挖掘、机器学习、统计机器翻译等领域的算法。

更多关于MapReduce具体应用的讨论可以在其他地方找到[8，16，7]。

---



### 2.2 Types

Even though the previous pseudocode is written in terms of string inputs and outputs, conceptually the map and reduce functions supplied by the user have associated types.

尽管前面的伪代码是用字符串输入和输出来写的，但从概念上讲，用户提供的map和reduce函数都有关联类型。

---



```
map (k1,v1) → list(k2,v2)
reduce (k2,list(v2)) → list(v2)
```

That is, the input keys and values are drawn from a different domain than the output keys and values. 

Furthermore, the intermediate keys and values are from the same domain as the output keys and values.

也就是说，输入键和值与输出键和值来自不同的域。

此外，中间键和值与输出键和值来自同一领域。

---



## 3 Implementation

Many different implementations of the MapReduce interface are possible. 

The right choice depends on the environment. 

For example, one implementation may be suitable for a small shared-memory machine, another for a large NUMA multiprocessor, and yet another for an even larger collection of networked machines. 

Since our original article, several open source implementations of MapReduce have been developed [1, 2], and the applicability of MapReduce to a variety of problem domains has been studied [7, 16].

MapReduce接口有许多不同的实现方式。

正确的选择取决于环境。

例如，一种实现可能适用于小型共享内存机器，另一种实现可能适用于大型NUMA多处理器，还有一种实现可能适用于更大的网络机器集合。

自我们最初的文章以来，已经开发了几个MapReduce的开源实现[1，2]，并且已经研究了MapReduce对各种问题领域的适用性[7，16]。

---

This section describes our implementation of MapReduce that is targeted to the computing environment in wide use at Google: large clusters of commodity PCs connected together with switched Gigabit Ethernet[4].

In our environment, machines are typically dual-processor x86 processors running Linux, with 4-8GB of memory per machine.

Individual machines typically have 1 gigabit/second of network bandwidth, but the overall bisection bandwidth available per machine is considerably less than 1 gigabit/second. 

A computing cluster contains many thousands of machines, and therefore machine failures are common.

Storage is provided by inexpensive IDE disks attached directly to individual machines.

GFS, a distributed file system developed in-house [10], is used to manage the data stored on these disks. 

The file system uses replication to provide availability and reliability on top of unreliable hardware

本节介绍我们针对Google广泛使用的计算环境：大型集群的MapReduce的实现。
用交换式千兆以太网连接在一起的商品电脑的比例。
[4].

在我们的环境中，机器一般是双处理器x86
运行Linux的处理器，每台机器有4-8GB的内存。

单台机器的网络带宽一般为1千兆/秒，但每台机器的总体可利用的二分频带宽远远小于1千兆/秒。

一个计算集群包含许多 数千台机器，因此机器故障是很常见的。

存储是由直接连接到各个机器上的廉价IDE磁盘提供的。

GFS是一个内部开发的分布式文件系统[10]。用于管理存储在这些磁盘上的数据。

文件系统在不可靠的硬件上使用复制来提供可用性和可靠性。

---

Users submit jobs to a scheduling system. 
Each job consists of a set of tasks, and is mapped by the scheduler to a set of available machines within a cluster.

用户向调度系统提交作业。
每个作业由一组任务组成，并由调度器映射到集群内的一组可用机器上。

---

### **3.1 Execution Overview**

The map invocations are distributed across multiple machines by automatically partitioning the input data into a set of M splits. 

The input splits can be processed in parallel by different machines.

Reduce invocations are distributed by partitioning the intermediate key space into R pieces using a partitioning function (e.g., hash(key) mod R).

The number of partitions (R) and the partitioning function are specified by the user.

通过自动将输入数据分割成一组M个分割，将地图调用分布在多台机器上。

输入的分裂数据可以由不同的机器并行处理。

Reduce调用是通过使用分割函数（例如，hash(key) mod R）将中间的密钥空间分割成R块来分布的。

分割的数量（R）和分割函数由用户指定。

---

Figure 1 shows the overall flow of a MapReduce operation in our implementation. 

When the user program calls the MapReduce function, the following sequence of actions occurs (the numbered labels in Figure 1 correspond to the numbers in the following list).

图1显示了我们实现中MapReduce操作的整体流程。

当用户程序调用MapReduce函数时，会发生以下一系列操作（图1中的数字标签对应以下列表中的数字）。

---

1 

The MapReduce library in the user program first splits the input files into M pieces of typically 16-64MB per piece (controllable by the user via an optional parameter).

It then starts up many copies of the program on a cluster of machines.

用户程序中的MapReduce库首先将输入文件分割成M块，每块通常为16-64MB（用户可通过一个可选参数进行控制）。

然后，它在机器集群上启动许多程序副本。

---

2

 One of the copies of the program—the master— is special.

The rest are workers that are assigned work by the master. 

There are M map tasks and R reduce tasks to assign. 

The master picks idle workers and assigns each one a map task or a reduce task.

其中一份程序的副本--母版--是特别的。

其余的是由主程序分配工作的工人。

有M个map任务和R个reduce任务要分配。

主程序挑选闲置的工人，给每个人分配一个map任务或reduce任务。

---

3

A worker who is assigned a map task reads the contents of the corresponding input split.

It parses key/value pairs out of the input data and passes each pair to the user-defined map function. 

The intermediate key/value pairs produced by the map function are buffered in memory.

被分配到地图任务的工作者会读取相应的输入拆分内容。

它从输入数据中解析出键/值对，并将每个键/值对传递给用户定义的地图函数。

地图函数产生的中间键/值对被缓冲在内存中。

---

4 

Periodically, the buffered pairs are written to local disk, partitioned into R regions by the partitioning function. 

The locations of these buffered pairs on the local disk are passed back to the master who is responsible for forwarding these locations to the reduce workers.

 周期性地将缓冲对写入本地磁盘，通过分区函数将其分割成R区域。

这些缓冲对在本地磁盘上的位置被传回给主盘，主盘负责将这些位置转发给减工。

---

5

When a reduce worker is notified by the master about these locations, it uses remote procedure calls to read the buffered data from the local disks of the map workers. 

When a reduce worker has read all intermediate data for its partition, it sorts it by the intermediate keys so that all occurrences of the same key are grouped together.

The sorting is needed because typically many different keys map to the same reduce task.

If the amount of intermediate data is too large to fit in memory, an external sort is used.

当主站通知reduce worker关于这些位置的信息时，它使用远程过程调用从 地图工作者的本地磁盘。

当一个还原工作者读取了 分区的所有中间数据，它按中间数据进行排序。键，以便将同一键的所有出现都归为一组。

之所以需要排序，是因为通常许多不同的键映射到 同样的reduce任务。

如果中间数据量过大的话 为了适应内存，使用了外部排序。

---

6

The reduce worker iterates over the sorted intermediate data and for each unique intermediate key encountered, it passes the key and the corresponding set of intermediate values to the user’s reduce function. 

The output of the reduce function is appended to a final output file for this reduce partition.

reduce worker对排序后的中间数据进行迭代，对于遇到的每一个唯一的中间键，它都会将键和对应的中间值集传递给用户的reduce函数。

减少函数的输出被追加到这个减少分区的最终输出文件中。

---

7

When all map tasks and reduce tasks have been completed, the master wakes up the user program. 

At this point, the MapReduce call in the user program returns back to the user code.

当所有的map任务和reduce任务完成后，主程序唤醒用户程序。

此时，用户程序中的MapReduce调用会回到用户代码中。

---



After successful completion, the output of the mapreduce execution is available in the R output files (one per reduce task, with file names specified by the user). 

Typically, users do not need to combine these R output files into one file; they often pass these files as input to another
MapReduce call or use them from another distributed application that is able to deal with input that is partitioned into multiple files.

成功完成后，mapreduce执行的输出结果可以在R输出文件中获得（每个reduce任务一个，文件名由用户指定）。

通常情况下，用户不需要将这些R输出文件合并成一个文件，他们经常将这些文件作为输入传给另一个
MapReduce调用或从另一个能够处理被分割成多个文件的输入的分布式应用中使用它们。

### **3.2 Master Data Structures**

The master keeps several data structures. For each map task and reduce task, it stores the state (idle, in-progress, or completed) and the identity of the worker machine (for nonidle tasks).

主站保留了多个数据结构。对于每个map任务和reduce任务，它存储了状态（空闲、进行中或已完成）和工作机的标识（对于非空闲任务）。

The master is the conduit through which the location of intermediate file regions is propagated from map tasks to reduce tasks. 

Therefore, for each completed map task, the master stores the locations and sizes of the R intermediate file regions produced by the map task.

Updates to this location and size information are received as map tasks are completed. The information is pushed incrementally to workers that have inprogress reduce tasks.

主站是中间文件区域的位置从地图任务传播到减少任务的渠道。

因此，对于每一个完成的地图任务，主站都会存储该地图任务产生的R中间文件区域的位置和大小。

随着地图任务的完成，会收到这个位置和大小信息的更新。这些信息会被递增地推送给有正在进行的reduce任务的工作者。

---

### **3.3 Fault Tolerance**

Since the MapReduce library is designed to help process very large amounts of data using hundreds or thousands of machines, the library must tolerate machine failures gracefully.

由于MapReduce库的设计是为了帮助处理使用数百或数千台机器的非常大量的数据，因此该库必须优雅地容忍机器故障。

**Handling Worker Failures**

The master pings every worker periodically. 

If no response is received from a worker in a certain amount of time, the master marks the worker as failed.

Any map tasks completed by the worker are reset back to their initial idle state and therefore become eligible for scheduling on other workers.

Similarly, any map task or reduce task in progress on a failed worker is also reset to idle and becomes eligible for rescheduling.

主站会定期对每个工人进行ping。

如果在一定时间内没有收到工人的响应，主站就会将该工人标记为失败。

该工人完成的任何地图任务都会被重置为初始空闲状态，因此有资格对其他工人进行调度。

同样，失败的工人上正在进行的任何地图任务或减少任务也会被重置为空闲状态，并有资格重新安排。

---

Completed map tasks are reexecuted on a failure because their output is stored on the local disk(s) of the failed machine and is therefore inaccessible.

Completed reduce tasks do not need to be reexecuted since their output is stored in a global file system.

已完成的映射任务会在故障时重新执行，因为它们的输出存储在故障机器的本地磁盘上，因此无法访问。

完成的reduce任务不需要重新执行，因为它们的输出存储在全局文件系统中。

---

When a map task is executed first by worker A and then later executed by worker B (because A failed), all workers executing reduce tasks are notified of the reexecution. Any reduce task that has not already read the data from worker A will read the data from worker B.

当一个map任务先由工人A执行，后由工人B执行（因为A失败了），所有执行reduce任务的工人都会被通知重新执行。任何还没有从工人A读取数据的reduce任务都会从工人B读取数据。

---

MapReduce is resilient to large-scale worker failures. For example, during one MapReduce operation, network maintenance on a running cluster was causing groups of 80 machines at a time to become unreachable for several minutes. 

The MapReduce master simply re executed the
work done by the unreachable worker machines and continued to make forward progress, eventually completing the MapReduce operation.

MapReduce对大规模工人故障具有弹性。例如，在一次MapReduce操作过程中，运行中的集群的网络维护导致一次80台机器的群组在几分钟内无法访问。

MapReduce主站只需重新执行
无法到达的工人机器所做的工作，并继续向前推进，最终完成了MapReduce操作。

---

**Semantics in the Presence of Failures**

When the user-supplied map and reduce operators are deterministic functions of their input values, our distributed implementation produces the same output as would have been produced by a nonfaulting sequential execution of the entire program.

当用户提供的map和reduce运算符是其输入值的确定性函数时，我们的分布式实现产生的输出与整个程序的无故障顺序执行所产生的输出相同。

---

We rely on atomic commits of map and reduce task outputs to achieve this property.

Each in-progress task writes its output to private temporary files.

A reduce task produces one such file, and a map task produces R such files (one per reduce task). 

When a map task completes, the worker sends a message to the master and includes the names of the R temporary files in the message. 

If the master receives a completion message for an already completed map task, it ignores the message.

Otherwise, it records the names of R files in a master data structure.

我们依靠map和reduce任务输出的原子提交来实现这一特性。

每个进行中的任务都会将其输出写入私有的临时文件中。

一个reduce任务会产生一个这样的文件，而一个map任务则会产生一个
产生R这样的文件（每个reduce任务一个）。

当一个map任务完成时，worker会向master发送一条消息，并包含了名字为 的R临时文件。

如果主站收到一个已经完成的地图任务的完成消息，它将忽略该消息。

否则，它将在主数据结构中记录R文件的名称。

---

When a reduce task completes, the reduce worker atomically renames its temporary output file to the final output file. 

If the same reduce task is executed on multiple machines, multiple rename calls will be executed for the same final output file. 

We rely on the atomic rename operation provided by the underlying file system to guarantee that the final file system
state contains only the data produced by one execution of the reduce task.

当一个reduce任务完成时，reduce worker会原子性地将其临时输出文件重命名为最终输出文件。

如果同一个reduce任务在多台机器上执行，则会对同一个最终输出文件执行多次重命名调用。

我们依靠底层文件系统提供的原子重命名操作来保证最终文件系统的
状态只包含一次执行reduce任务所产生的数据。

---

The vast majority of our map and reduce operators are deterministic, and the fact that our semantics are equivalent to a sequential execution in this case makes it very easy for programmers to reason about their
program’s behavior. 

When the map and/or reduce operators are nondeterministic, we provide weaker but still reasonable semantics.

In the presence of nondeterministic operators, the output of a particular
reduce task R1 is equivalent to the output for R1 produced by a sequential execution of the nondeterministic program. 

However, the output for a different reduce task R2 may correspond to the output for R2 produced by a different sequential execution of the nondeterministic program.

我们绝大多数的映射和还原运算符都是确定性的。以及我们的语义等同于顺序执行的事实。在这种情况下，程序员可以很容易地对他们的 程序的行为。

当映射和/或还原运算符是非确定性的，我们提供了较弱的但仍然合理的语义。

在 非确定性算子的存在，某一个特定的
reduce任务R1相当于非确定性程序的顺序执行所产生的R1的输出。

然而，R1的输出是 不同的缩减任务R2可能对应于R2的输出。由不同的顺序执行非确定性程序。

---

Consider map task M and reduce tasks R1 and R2. Let e(Ri) be the execution of R1 that committed (there is exactly one such execution).

The weaker semantics arise because e(R1) may have read the output produced by one execution of M, and e(R2) may have read the output produced by a different execution of M.

考虑地图任务M，减少任务R1和R2。让e(Ri)是R1提交的执行（正好有一个这样的执行）。

产生较弱的语义是因为e(R1)可能读取了M的一个执行产生的输出，而e(R2)可能读取了M的另一个执行产生的输出。

---

### **3.4 Locality**

Network bandwidth is a relatively scarce resource in our computing environment. 

We conserve network bandwidth by taking advantage of the fact that the input data (managed by GFS [10]) is stored on the local disks of the machines that make up our cluster. 

GFS divides each file into 64MB blocks and stores several copies of each block (typically 3 copies) on different machines. 

The MapReduce master takes the location information of the input files into account and attempts to schedule a map task on a machine that contains a replica of the corresponding input data.

Failing that, it attempts to schedule a map task near a replica of that task’s input data (e.g., on a worker machine that is on the same network switch as the machine containing the data). 

When running large MapReduce operations on a significant fraction of the workers in a cluster, most input data is read locally and consumes no network bandwidth.

在我们的计算环境中，网络带宽是一种比较稀缺的资源。

我们通过利用以下方式来节约网络带宽 事实上，输入数据(由GFS[10]管理)是存储在本地系统上的。 组成我们集群的机器的磁盘。

GFS将每个文件 成64MB的区块，并将每个区块存储若干份（一般为3份）。副本）在不同的机器上。

MapReduce 主站将输入文件的位置信息考虑在内，并尝试调度这些文件。的副本的机器上的映射任务。输入数据。

如果失败，它就会尝试在副本附近安排一个地图任务。的输入数据（例如，在同一台工人机器上的输入数据）。网络交换机作为包含数据的机器）。) 

当运行大型 对集群中相当一部分工作者进行MapReduce操作，大部分输入数据在本地读取，不消耗网络带宽。

### **3.5 Task Granularity**

We subdivide the map phase into M pieces and the reduce phase into R pieces as described previously. Ideally, M and R should be much larger than the number of worker machines. 

Having each worker perform many different tasks improves dynamic load balancing and also speeds up recovery when a worker fails: the many map tasks it has completed can be spread out across all the other worker machines.

我们将映射阶段细分为M个片段，将还原阶段细分为R个片段，如前所述。理想情况下，M和R应该远大于worker机器的数量。

让每个worker执行许多不同的任务可以改善动态负载平衡，也可以在一个worker故障时加快恢复速度：它所完成的许多map任务可以分散到所有其他worker机器上。

---

There are practical bounds on how large M and R can be in our implementation since the master must make O(M+R) scheduling decisions and keep O(M*R) state in memory as described. 

(The constant factors for memory usage are small, however. The O(M*R) piece of the state consists of approximately one byte of data per map task/ reduce task pair.)

在我们的实现中，M和R可以有多大的实际界限，因为主站必须做出O(M+R)的调度决策，并在内存中保持O(M*R)的状态，如所述。

(不过，内存使用的常数因素很小。O(M*R)片的状态大约由每个映射任务/减少任务对的一个字节数据组成）。)

---

Furthermore, R is often constrained by users because the output of each reduce task ends up in a separate output file.

In practice, we tend to choose M so that each individual task is roughly 16MB to 64MB of input data (so that the locality optimization described previously is most effective), and we make R a small multiple of the number of worker
machines we expect to use. 

We often perform MapReduce computations with M=200,000 and R=5,000, using 2,000 worker machines.

此外，R经常受到用户的限制，因为每个reduce任务的输出最终都会在一个单独的输出文件中。

在实践中，我们倾向于选择M，使每个独立任务的输入数据大概在16MB到64MB之间（这样前面描述的locality优化是最有效的），我们使R成为worker数量的一个小倍数。
我们期望使用的机器。

我们经常用M=200,000，R=5,000，使用2,000台工人机进行MapReduce计算。

---

### **3.6 Backup Tasks**

One of the common causes that lengthens the total time taken for a MapReduce operation is a straggler, that is, a machine that takes an unusually long time to complete one of the last few map or reduce tasks in the computation.

Stragglers can arise for a whole host of reasons. For example, a machine with a bad disk may experience frequent correctable errors that slow its read performance from 30MB/s to 1MB/s.

The cluster scheduling system may have scheduled other tasks on the machine, causing it to execute the MapReduce code more slowly due to competition for CPU, memory, local disk, or network bandwidth.

A recent problem we experienced was a bug in machine initialization code that caused processor caches to be disabled: computations on affected machines slowed down by over a factor of one hundred.

常见的原因之一是，延长了总的时间。MapReduce操作是一个杂牌机，也就是说，一个机器把一个 超长时间完成最后几张地图之一或减少任务 在计算中。

散兵游勇的出现有很多原因。因为 例如，具有坏磁盘的机器可能会经历频繁的可纠正错误，使其读取性能从30MB/s慢到1MB/s。

群集调度系统可能已经将其他任务安排在了? 机器，导致其执行MapReduce代码的速度更慢，原因在于 以争夺CPU、内存、本地磁盘或网络带宽。

最近我们遇到的问题是机器初始化中的一个错误 导致处理器缓存被禁用的代码：在 受影响的机器速度慢了一百多倍。

---

We have a general mechanism to alleviate the problem of stragglers. When a MapReduce operation is close to completion, the master schedules backup executions of the remaining in-progress tasks.

The task is marked as completed whenever either the primary or the backup execution completes. 

We have tuned this mechanism so that it typically increases the computational resources used by the operation by no more than a few percent.

We have found that this significantly reduces the time to complete large MapReduce operations.

As an example, the sort program described in Section 5.3 takes 44% longer to complete when the backup task mechanism is disabled.

我们有一个通用的机制来缓解stragglers的问题。

当一个MapReduce操作接近完成时，主控的 安排剩余的进行中任务的备份执行。

该 每当主任务或主任务中的任何一个任务被标记为完成时，该任务就会被标记为完成。 备份执行完成。

我们对这个机制进行了调整，使其 通常会增加操作所使用的计算资源。不超过百分之几。

我们发现，这大大 减少了完成大型MapReduce操作的时间。

作为一个 例如，第5.3节中描述的排序程序需要44%的时间。 当备份任务机制被禁用时，要完成。

