# MapReduce: Simplified data processing on large cluster 



## Abstract



MapReduce is a programming model and an associated implementation for processing and generating large datasets that is amenable to a broad variety of real-world tasks.  Users specify the computation in terms of a map and a reduce function, and the underlying runtime system automatically parallelizes the computation across large-scale clusters of machines, handles machine failures, and schedules intermachine communication to make efficient use of the network and disks. Programmers find the system  easy to use: more than ten thousand distinct MapReduce programs have been implemented internally at Google over the past four years, and an average of one hundred thousand MapReduce jobs are executed on Google’s clusters every day, processing a total of more than twenty petabytes of data per day.

MapReduce是一种编程模型和相关的实现，用于处理和生成大型数据集，适用于各种现实世界的任务。 用户用地图和reduce函数来指定计算，底层的运行时系统会自动在大规模的机器集群中并行计算，处理机器故障，并调度机器间的通信，以有效利用网络和磁盘。程序员们发现这个系统很容易使用：在过去的四年里，谷歌内部已经实现了一万多个不同的MapReduce程序，平均每天有十万个MapReduce作业在谷歌的集群上执行，每天处理的数据总量超过二十PB。

---

## 1 Introduction

Prior to our development of MapReduce, the authors and many others at Google implemented hundreds of special-purpose computations that process large amounts of raw data, such as crawled documents, Web request logs, etc., to compute various kinds of derived data, such as inverted indices, various representations of the graph structure of Web documents, summaries of the number of pages crawled per host, and the set of most frequent queries in a given day. Most such computations are conceptually straightforward. However, the input data is usually large and the computations have to be distributed across hundreds or thousands of machines in order to finish in a reasonable amount of time. The issues of how to parallelize the computation, distribute the data, and handle failures conspire to obscure the original simple computation with large amounts of complex code to deal with these issues.

在我们开发MapReduce之前，作者和Google的许多其他人实现了数百种特殊用途的计算，这些计算处理了大量的原始数据，如抓取的文档、Web请求日志等，计算出各种衍生数据，如倒置索引、Web文档的图结构的各种表示、每个主机抓取的页面数量的汇总，以及某一天最频繁的查询集。大多数这样的计算在概念上是简单的。然而，输入的数据通常很大，计算必须分布在数百或数千台机器上，才能在合理的时间内完成。如何并行化计算、分布数据、处理故障等问题，合谋着用大量复杂的代码来处理这些问题，掩盖了原本简单的计算。

---

As a reaction to this complexity, we designed a new abstraction that allows us to express the simple computations we were trying to perform but hides the messy details of parallelization, fault tolerance, data distribution and load balancing in a library.  Our abstraction is inspired by the map and reduce primitives present in Lisp and many other functional languages. We realized that most of our computations involved applying a map operation to each logical record’ in our input in order to compute a set of intermediate key/value pairs, and then applying a reduce operation to all the values that shared the same key in order to combine the derived data appropriately.  Our use of a functional model with user-specified map and reduce operations allows us to parallelize large computations easily and to use reexecution as the primary mechanism for fault tolerance.

作为对这种复杂性的反应，我们设计了一个新的抽象，它允许我们表达我们试图执行的简单计算，但在一个库中隐藏了并行化、容错、数据分配和负载平衡的混乱细节。 我们的抽象受Lisp和其他许多函数式语言中的映射和还原基元的启发。我们意识到，我们的大部分计算涉及到对输入中的每条逻辑记录应用map操作，以计算一组中间键/值对，然后对所有共享相同键的值应用reduce操作，以适当地组合衍生数据。 我们使用带有用户指定的映射和还原操作的功能模型，使我们能够轻松地并行处理大型计算，并将重新执行作为容错的主要机制。

---

The major contributions of this work are a simple and powerful interface that enables automatic parallelization and distribution of large-scale computations, combined with an implementation of this interface that achieves high performance on large clusters of commodity PCs.  The programming model can also be used to parallelize computations across multiple cores of the same machine.

这项工作的主要贡献是提供了一个简单而强大的接口，可以实现大规模计算的自动并行化和分配，结合这个接口的实现，可以在大型商品PC集群上实现高性能。 该编程模型还可用于在同一台机器的多个核心上进行并行化计算。

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

The computation takes a set of input key/value pairs, and produces a set of output key/value pairs.  The user of the MapReduce library expresses the computation as two functions: map and reduce.

计算需要一组输入键/值对，并产生一组输出键/值对。 MapReduce库的用户将计算表达为两个函数：map和reduce。

---

Map, written by the user, takes an input pair and produces a set of intermediate key/value pairs.  The MapReduce library groups together all intermediate values associated with the same intermediate key and passes them to the reduce function.

Map，由用户编写，接受一个输入对并产生一组中间键/值对。 MapReduce库将所有与同一个中间键相关联的中间值分组，并将它们传递给reduce函数。

---

The reduce function, also written by the user, accepts an intermediate key and a set of values for that key.It merges these values together to form a possibly smaller set of values.  Typically just zero or one output value is produced per reduce invocation.  The intermediate values are supplied to the user’s reduce function via an iterator.  This allows us to handle lists of values that are too large to fit in memory.

reduce函数也是由用户编写的，它接受一个中间键和该键的一组值，并将这些值合并在一起，形成一个可能较小的值集。 通常情况下，每次reduce调用只产生零或一个输出值。 中间值通过迭代器提供给用户的 reduce 函数。 这使得我们可以处理那些太大而无法放入内存的值列表。

---

### 2.1 Example

Consider the problem of counting the number of occurrences of each word in a large collection of documents.  The user would write code similar to the following pseudocode.

考虑计算一个大型文档集合中每个单词的出现次数的问题。 用户将编写类似于以下伪代码的代码。

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

The map function emits each word plus an associated count of occurrences (just 1 in this simple example).  The reduce function sums together all counts emitted for a particular word.

map 函数发出每个单词加上相关的出现次数（在这个简单的例子中只有 1 次）。 reduce函数将某一特定单词的所有计数相加。

---

In addition, the user writes code to fill in a mapreduce specification object with the names of the input and output files and optional tuning parameters.  The user then invokes the MapReduce function, passing it to the specification object. The user’s code is linked together with the MapReduce library (implemented in C++).  Our original MapReduce paper contains the full program text for this example [8]

此外，用户编写代码，在mapreduce规范对象中填写输入和输出文件的名称以及可选的调整参数。 然后用户调用MapReduce函数，将其传递给规范对象。用户的代码与MapReduce库（用C++实现）链接在一起。 我们最初的MapReduce论文包含了这个例子的完整程序文本[8] 。

---

More than ten thousand distinct programs have been implemented using MapReduce at Google, including algorithms for large-scale graph processing, text processing, data mining, machine learning, statistical machine translation, and many other areas.  More discussion of specific applications of MapReduce can be found elsewhere [8, 16, 7].

在Google，已经有一万多个不同的程序使用MapReduce实现，包括大规模图处理、文本处理、数据挖掘、机器学习、统计机器翻译等许多领域的算法。 更多关于MapReduce具体应用的讨论可以在其他地方找到[8，16，7]。

---



### 2.2 Types

Even though the previous pseudocode is written in terms of string inputs and outputs, conceptually the map and reduce functions supplied by the user have associated types.

尽管前面的伪代码是用字符串输入和输出来写的，但从概念上讲，用户提供的map和reduce函数都有关联类型。

---



```
map (k1,v1) → list(k2,v2)
reduce (k2,list(v2)) → list(v2)
```

That is, the input keys and values are drawn from a different domain than the output keys and values. Furthermore, the intermediate keys and values are from the same domain as the output keys and values.

也就是说，输入键和值与输出键和值来自不同的域。此外，中间键和值与输出键和值来自同一领域。

---



## 3 Implementation

Many different implementations of the MapReduce interface are possible. The right choice depends on the environment. For example, one implementation may be suitable for a small shared-memory machine, another for a large NUMA multiprocessor, and yet another for an even larger collection of networked machines. Since our original article, several open source implementations of MapReduce have been developed [1, 2], and the applicability of MapReduce to a variety of problem domains has been studied [7, 16].

MapReduce接口有许多不同的实现方式。正确的选择取决于环境。例如，一种实现可能适用于小型共享内存机器，另一种实现可能适用于大型NUMA多处理器，还有一种实现可能适用于更大的网络机器集合。自我们最初的文章以来，已经开发了几个MapReduce的开源实现[1，2]，并且已经研究了MapReduce对各种问题领域的适用性[7，16]。。

---

This section describes our implementation of MapReduce that is targeted to the computing environment in wide use at Google: large clusters of commodity PCs connected together with switched Gigabit Ethernet[4]. In our environment, machines are typically dual-processor x86 processors running Linux, with 4-8GB of memory per machine. Individual machines typically have 1 gigabit/second of network bandwidth, but the overall bisection bandwidth available per machine is considerably less than 1 gigabit/second.  A computing cluster contains many thousands of machines, and therefore machine failures are common. Storage is provided by inexpensive IDE disks attached directly to individual machines. GFS, a distributed file system developed in-house [10], is used to manage the data stored on these disks. The file system uses replication to provide availability and reliability on top of unreliable hardware

本节介绍了我们针对Google广泛使用的计算环境的MapReduce的实现：用交换式千兆以太网连接在一起的大型商品PC集群[4]。在我们的环境中，机器通常是运行Linux的双处理器x86处理器，每台机器有4-8GB的内存。单台机器的网络带宽一般为1千兆/秒，但每台机器的整体可用的二分频带宽远远小于1千兆/秒。 一个计算集群包含成千上万台机器，因此机器故障很常见。存储是由直接连接到各个机器上的廉价IDE磁盘提供的。GFS是一个内部开发的分布式文件系统[10]，用于管理存储在这些磁盘上的数据。该文件系统在不可靠的硬件上使用复制来提供可用性和可靠性。

---

Users submit jobs to a scheduling system.  Each job consists of a set of tasks, and is mapped by the scheduler to a set of available machines within a cluster.

用户向调度系统提交作业。 每个作业由一组任务组成，并由调度器映射到集群内的一组可用机器上。

---

### **3.1 Execution Overview**

The map invocations are distributed across multiple machines by automatically partitioning the input data into a set of M splits.  The input splits can be processed in parallel by different machines. Reduce invocations are distributed by partitioning the intermediate key space into R pieces using a partitioning function (e.g., hash(key) mod R). The number of partitions (R) and the partitioning function are specified by the user.

通过自动将输入数据分割成一组M个分割，将地图调用分布在多台机器上。 输入的分裂数据可以由不同的机器并行处理。Reduce调用是通过使用分区函数（例如，hash(key) mod R）将中间的密钥空间分割成R块来分布的。分割的数量（R）和分割函数由用户指定。

---

Figure 1 shows the overall flow of a MapReduce operation in our implementation.  When the user program calls the MapReduce function, the following sequence of actions occurs (the numbered labels in Figure 1 correspond to the numbers in the following list).

图1显示了我们实现中MapReduce操作的整体流程。 当用户程序调用MapReduce函数时，会发生以下一系列操作（图1中的数字标签对应以下列表中的数字）。

---

1 

The MapReduce library in the user program first splits the input files into M pieces of typically 16-64MB per piece (controllable by the user via an optional parameter). It then starts up many copies of the program on a cluster of machines.

用户程序中的MapReduce库首先将输入文件分割成M块，每块通常为16-64MB（用户可通过一个可选参数控制）。然后，它在机器集群上启动许多程序副本。

---

2

One of the copies of the program—the master— is special. The rest are workers that are assigned work by the master. There are M map tasks and R reduce tasks to assign.  The master picks idle workers and assigns each one a map task or a reduce task.

其中一个程序的副本--主程序--是特殊的。其余的是由主程序分配工作的工人。有M个map任务和R个reduce任务要分配。 主程序挑选闲置的工人，给每个人分配一个map任务或reduce任务。

---

3

A worker who is assigned a map task reads the contents of the corresponding input split. It parses key/value pairs out of the input data and passes each pair to the user-defined map function. The intermediate key/value pairs produced by the map function are buffered in memory.

被分配到地图任务的工作者读取相应的输入拆分的内容，它从输入数据中解析出键/值对，并将每个键/值对传递给用户定义的地图函数。它从输入数据中解析出键/值对，并将每个键/值对传递给用户定义的映射函数。地图函数产生的中间键/值对被缓冲在内存中。

---

4 

Periodically, the buffered pairs are written to local disk, partitioned into R regions by the partitioning function.  The locations of these buffered pairs on the local disk are passed back to the master who is responsible for forwarding these locations to the reduce workers.

周期性地将缓冲对写入本地磁盘，通过分区函数将其分割成R区域。 这些缓冲对在本地磁盘上的位置被传回给主盘，主盘负责将这些位置转发给减工。

---

5

When a reduce worker is notified by the master about these locations, it uses remote procedure calls to read the buffered data from the local disks of the map workers. When a reduce worker has read all intermediate data for its partition, it sorts it by the intermediate keys so that all occurrences of the same key are grouped together. The sorting is needed because typically many different keys map to the same reduce task. If the amount of intermediate data is too large to fit in memory, an external sort is used.

当主站通知reduce worker这些位置时，它使用远程过程调用从map worker的本地磁盘中读取缓冲数据。当一个reduce worker读取了它的分区的所有中间数据后，它就会按照中间键对数据进行排序，以便将所有出现在同一键上的数据分组。之所以需要进行排序，是因为通常许多不同的键会映射到同一个reduce任务。如果中间数据量太大，内存中无法容纳，则使用外部排序。

---

6

The reduce worker iterates over the sorted intermediate data and for each unique intermediate key encountered, it passes the key and the corresponding set of intermediate values to the user’s reduce function.  The output of the reduce function is appended to a final output file for this reduce partition.

reduce worker对排序后的中间数据进行迭代，对于遇到的每一个唯一的中间键，它都会将键和对应的中间值集传递给用户的reduce函数。 reduce函数的输出被追加到这个reduce分区的最终输出文件中。

---

7

When all map tasks and reduce tasks have been completed, the master wakes up the user program.  At this point, the MapReduce call in the user program returns back to the user code.

当所有map任务和reduce任务完成后，主程序唤醒用户程序。 此时，用户程序中的MapReduce调用会回到用户代码中。

---



After successful completion, the output of the mapreduce execution is available in the R output files (one per reduce task, with file names specified by the user).  Typically, users do not need to combine these R output files into one file; they often pass these files as input to another MapReduce call or use them from another distributed application that is able to deal with input that is partitioned into multiple files.

成功完成后，mapreduce执行的输出结果可以在R输出文件中获得（每个reduce任务一个，文件名由用户指定）。 通常情况下，用户不需要将这些R输出文件合并成一个文件，他们通常将这些文件作为输入传递给另一个MapReduce调用，或者从另一个分布式应用中使用这些文件，这些应用能够处理被分割成多个文件的输入。

### **3.2 Master Data Structures**

The master keeps several data structures. For each map task and reduce task, it stores the state (idle, in-progress, or completed) and the identity of the worker machine (for nonidle tasks).

主站保留了多个数据结构。对于每个map任务和reduce任务，它存储了状态（空闲、进行中或已完成）和工作机的标识（对于非空闲任务）。

The master is the conduit through which the location of intermediate file regions is propagated from map tasks to reduce tasks.  Therefore, for each completed map task, the master stores the locations and sizes of the R intermediate file regions produced by the map task. Updates to this location and size information are received as map tasks are completed. The information is pushed incrementally to workers that have inprogress reduce tasks.

主站是将中间文件区域的位置从地图任务传播到减少任务的渠道。 因此，对于每一个完成的地图任务，主站都会存储该地图任务产生的R中间文件区域的位置和大小。随着地图任务的完成，会收到这个位置和大小信息的更新。这些信息会被递增地推送给有正在进行的reduce任务的工作者。

---

### **3.3 Fault Tolerance**

Since the MapReduce library is designed to help process very large amounts of data using hundreds or thousands of machines, the library must tolerate machine failures gracefully.

由于MapReduce库的设计是为了帮助处理使用数百或数千台机器的非常大量的数据，因此该库必须优雅地容忍机器故障。

**Handling Worker Failures**

The master pings every worker periodically. If no response is received from a worker in a certain amount of time, the master marks the worker as failed. Any map tasks completed by the worker are reset back to their initial idle state and therefore become eligible for scheduling on other workers. Similarly, any map task or reduce task in progress on a failed worker is also reset to idle and becomes eligible for rescheduling.

主站会定期对每个工人进行ping，如果在一定时间内没有收到工人的响应，主站会将工人标记为失败。如果在一定时间内没有收到工人的响应，主站就会将该工人标记为失败。该工人完成的任何地图任务都会被重置为初始空闲状态，因此有资格对其他工人进行调度。同样，失败的工人上正在进行的任何地图任务或减少任务也会被重置为空闲状态，并有资格重新安排。

---

Completed map tasks are reexecuted on a failure because their output is stored on the local disk(s) of the failed machine and is therefore inaccessible. Completed reduce tasks do not need to be reexecuted since their output is stored in a global file system.

已完成的映射任务会在故障时被重新执行，因为它们的输出存储在故障机器的本地磁盘上，因此无法访问。完成的reduce任务不需要重新执行，因为它们的输出存储在全局文件系统中。

---

When a map task is executed first by worker A and then later executed by worker B (because A failed), all workers executing reduce tasks are notified of the reexecution. Any reduce task that has not already read the data from worker A will read the data from worker B.

当一个map任务先由工人A执行，后由工人B执行（因为A失败了），所有执行reduce任务的工人都会被通知重新执行。任何还没有从工人A读取数据的reduce任务都会从工人B读取数据。

---

MapReduce is resilient to large-scale worker failures. For example, during one MapReduce operation, network maintenance on a running cluster was causing groups of 80 machines at a time to become unreachable for several minutes. The MapReduce master simply re executed the work done by the unreachable worker machines and continued to make forward progress, eventually completing the MapReduce operation.

MapReduce对大规模工作者故障具有弹性。例如，在一次MapReduce操作过程中，运行中的集群的网络维护导致一次80台机器的群组在几分钟内无法访问。MapReduce主控只需重新执行无法到达的工作机所做的工作，并继续向前推进，最终完成了MapReduce操作。

---

**Semantics in the Presence of Failures**

When the user-supplied map and reduce operators are deterministic functions of their input values, our distributed implementation produces the same output as would have been produced by a nonfaulting sequential execution of the entire program.

当用户提供的map和reduce运算符是其输入值的确定性函数时，我们的分布式实现产生的输出与整个程序的无故障顺序执行所产生的输出相同。

---

We rely on atomic commits of map and reduce task outputs to achieve this property. Each in-progress task writes its output to private temporary files. A reduce task produces one such file, and a map task produces R such files (one per reduce task).  When a map task completes, the worker sends a message to the master and includes the names of the R temporary files in the message.  If the master receives a completion message for an already completed map task, it ignores the message. Otherwise, it records the names of R files in a master data structure.

我们依靠map和reduce任务输出的原子提交来实现这一特性。每个进行中的任务将其输出写入私有的临时文件中。一个reduce任务产生一个这样的文件，一个map任务产生R个这样的文件（每个reduce任务一个）。 当一个map任务完成时，worker会给master发送一个消息，并在消息中包含R个临时文件的名称。 如果主站收到一个已经完成的地图任务的完成消息，它就会忽略该消息。否则，它将R文件的名称记录在主数据结构中。

---

When a reduce task completes, the reduce worker atomically renames its temporary output file to the final output file.  If the same reduce task is executed on multiple machines, multiple rename calls will be executed for the same final output file.  We rely on the atomic rename operation provided by the underlying file system to guarantee that the final file system state contains only the data produced by one execution of the reduce task.

当一个reduce任务完成时，reduce worker会原子地将其临时输出文件重命名为最终输出文件。 如果同一个reduce任务在多台机器上执行，则会对同一个最终输出文件执行多次重命名调用。 我们依靠底层文件系统提供的原子重命名操作来保证最终的文件系统状态只包含一次执行reduce任务所产生的数据。

---

The vast majority of our map and reduce operators are deterministic, and the fact that our semantics are equivalent to a sequential execution in this case makes it very easy for programmers to reason about their program’s behavior.  When the map and/or reduce operators are nondeterministic, we provide weaker but still reasonable semantics. In the presence of nondeterministic operators, the output of a particular reduce task R1 is equivalent to the output for R1 produced by a sequential execution of the nondeterministic program. However, the output for a different reduce task R2 may correspond to the output for R2 produced by a different sequential execution of the nondeterministic program.

我们的绝大多数map和reduce操作符都是确定性的，在这种情况下，我们的语义等同于顺序执行，这使得程序员很容易推理出他们的 程序的行为。 当map和/或reduce运算符是非确定性的，我们提供了较弱但仍然合理的语义。在存在非确定性操作符的情况下，一个特定的 reduce任务R1的输出相当于由非确定性程序的顺序执行产生的R1的输出。然而，不同的reduce任务R2的输出可以对应于由非确定性程序的不同顺序执行产生的R2的输出。

---

Consider map task M and reduce tasks R1 and R2. Let e(Ri) be the execution of R1 that committed (there is exactly one such execution). The weaker semantics arise because e(R1) may have read the output produced by one execution of M, and e(R2) may have read the output produced by a different execution of M.

考虑映射任务M，减少任务R1和R2。让e(Ri)是R1提交的执行（正好有一个这样的执行）。产生较弱的语义是因为e(R1)可能读取了M的一个执行产生的输出，而e(R2)可能读取了M的另一个执行产生的输出。

---

### **3.4 Locality**

Network bandwidth is a relatively scarce resource in our computing environment. We conserve network bandwidth by taking advantage of the fact that the input data (managed by GFS [10]) is stored on the local disks of the machines that make up our cluster. GFS divides each file into 64MB blocks and stores several copies of each block (typically 3 copies) on different machines. The MapReduce master takes the location information of the input files into account and attempts to schedule a map task on a machine that contains a replica of the corresponding input data.Failing that, it attempts to schedule a map task near a replica of that task’s input data (e.g., on a worker machine that is on the same network switch as the machine containing the data). When running large MapReduce operations on a significant fraction of the workers in a cluster, most input data is read locally and consumes no network bandwidth.

在我们的计算环境中，网络带宽是一种相对稀缺的资源。我们利用输入数据(由GFS[10]管理)存储在组成我们集群的机器的本地磁盘上这一事实来节约网络带宽。GFS将每个文件划分为64MB的块，并在不同的机器上存储每个块的多个副本（通常为3份）。MapReduce主控会考虑到输入文件的位置信息，并尝试在包含相应输入数据副本的机器上调度一个地图任务。 如果做不到这一点，它就会尝试在该任务的输入数据副本附近调度一个地图任务（例如，在与包含数据的机器处于同一网络交换机上的工作机上）。当在集群中相当一部分工人机上运行大型MapReduce操作时，大部分输入数据都是本地读取的，不消耗网络带宽。

### **3.5 Task Granularity**

We subdivide the map phase into M pieces and the reduce phase into R pieces as described previously. Ideally, M and R should be much larger than the number of worker machines. Having each worker perform many different tasks improves dynamic load balancing and also speeds up recovery when a worker fails: the many map tasks it has completed can be spread out across all the other worker machines.

我们将映射阶段细分为M个片段，将还原阶段细分为R个片段，如前所述。理想情况下，M和R应该远大于worker机器的数量。让每个worker执行许多不同的任务可以改善动态负载平衡，也可以在一个worker故障时加快恢复速度：它所完成的许多map任务可以分散到所有其他worker机器上。

---

There are practical bounds on how large M and R can be in our implementation since the master must make O(M+R) scheduling decisions and keep O(M*R) state in memory as described. (The constant factors for memory usage are small, however. The O(M*R) piece of the state consists of approximately one byte of data per map task/ reduce task pair.)

在我们的实现中，M和R可以有多大的实际界限，因为主站必须做出O(M+R)的调度决策，并在内存中保持O(M*R)的状态，如所述。(不过，内存使用的常数因素很小。O(M*R)片的状态大约由每个映射任务/减少任务对的一个字节数据组成）。)

---

Furthermore, R is often constrained by users because the output of each reduce task ends up in a separate output file. In practice, we tend to choose M so that each individual task is roughly 16MB to 64MB of input data (so that the locality optimization described previously is most effective), and we make R a small multiple of the number of worker machines we expect to use.  We often perform MapReduce computations with M=200,000 and R=5,000, using 2,000 worker machines.

此外，R经常受到用户的限制，因为每个reduce任务的输出最终都会在一个单独的输出文件中。在实践中，我们倾向于选择M，使每个独立任务的输入数据大概在16MB到64MB之间（这样前面描述的locality优化才是最有效的），并且使R成为我们预计使用的工人机数量的一个小倍数。 我们经常在M=200,000，R=5,000的情况下执行MapReduce计算，使用2,000台工作机。

---

### **3.6 Backup Tasks**

One of the common causes that lengthens the total time taken for a MapReduce operation is a straggler, that is, a machine that takes an unusually long time to complete one of the last few map or reduce tasks in the computation. Stragglers can arise for a whole host of reasons. For example, a machine with a bad disk may experience frequent correctable errors that slow its read performance from 30MB/s to 1MB/s. The cluster scheduling system may have scheduled other tasks on the machine, causing it to execute the MapReduce code more slowly due to competition for CPU, memory, local disk, or network bandwidth. A recent problem we experienced was a bug in machine initialization code that caused processor caches to be disabled: computations on affected machines slowed down by over a factor of one hundred.

延长MapReduce操作所需总时间的常见原因之一是straggler，即一台机器需要异常长的时间来完成计算中最后几个map或reduce任务中的一个。Straggler的产生有很多原因。例如，一台磁盘坏了的机器可能会频繁出现可纠正的错误，使其读取性能从30MB/s慢到1MB/s。集群调度系统可能在机器上安排了其他任务，导致其因竞争CPU、内存、本地磁盘或网络带宽而执行MapReduce代码的速度更慢。我们最近遇到的一个问题是机器初始化代码中的一个bug，导致处理器缓存被禁用：受影响机器上的计算速度慢了一百多倍。

---

We have a general mechanism to alleviate the problem of stragglers. When a MapReduce operation is close to completion, the master schedules backup executions of the remaining in-progress tasks. The task is marked as completed whenever either the primary or the backup execution completes. We have tuned this mechanism so that it typically increases the computational resources used by the operation by no more than a few percent. We have found that this significantly reduces the time to complete large MapReduce operations. As an example, the sort program described in Section 5.3 takes 44% longer to complete when the backup task mechanism is disabled.

我们有一个通用的机制来缓解stragglers的问题。当一个MapReduce操作接近完成时，主控会对剩余的进行中任务进行备份执行调度。每当主任务或备份执行完成时，该任务就会被标记为完成。我们对这一机制进行了调整，使其通常增加的操作所使用的计算资源不超过百分之几。我们发现，这大大减少了完成大型MapReduce操作的时间。举个例子，当备份任务机制被禁用时，5.3节中描述的排序程序完成时间延长了44%。

---

### **4 Refinements**

Although the basic functionality provided by simply writing map and reduce functions is sufficient for most needs, we have found a few extensions useful. These include:

虽然简单地编写map和reduce函数所提供的基本功能已经足够满足大多数需求，但我们发现一些扩展是有用的。这些包括：

---

• user-specified partitioning functions for determining the mapping of intermediate key values to the R reduce shards; 

• ordering guarantees: Our implementation guarantees that within each of the R reduce partitions, the intermediate key/value pairs are processed in increasing key order; 

• user-specified combiner functions for doing partial combination of generated intermediate values with the same key within the same map task (to reduce the amount of intermediate data that must be transferred across the network); 

• custom input and output types, for reading new input formats and producing new output formats; 

• a mode for execution on a single machine for simplifying debugging and small-scale testing.

- 用户指定的分区函数，用于确定中间键值到R还原碎片的映射。
- 排序保证。我们的实现保证在每一个R还原分区中，中间键/值对都是按照增加键的顺序处理的。
- 用户指定的组合函数，用于在同一映射任务中对生成的具有相同键的中间值进行部分组合（以减少必须在网络上传输的中间数据量）。
- 自定义的输入和输出类型，用于读取新的输入格式和。产生新的输出格式。
- 在单机上执行的模式，以简化调试。和小规模测试。

---

The original article has more detailed discussions of each of these items [8].

原文对这些项目都有比较详细的讨论[8]。

## 5 Performance 

In this section, we measure the performance of MapReduce on two computations running on a large cluster of machines. One computation searches through approximately one terabyte of data looking for a particular pattern. The other computation sorts approximately one terabyte of data.

在本节中，我们将测量MapReduce在大型机器集群上运行的两个计算的性能。其中一个计算在大约一TB的数据中搜索一个特定的模式，另一个计算对大约一TB的数据进行排序。另一种计算是对大约一兆字节的数据进行排序。

---

These two programs are representative of a large subset of the real programs written by users of MapReduce—one class of programs shuffles data from one representation to another, and another class extracts a small amount of interesting data from a large dataset.

这两个程序代表了MapReduce用户编写的大量真实程序的子集--一类程序将数据从一个表示方式洗牌到另一个表示方式，另一类程序从一个大型数据集中提取少量有趣的数据。

---

### **5.1 Cluster Configuration**

All of the programs were executed on a cluster that consisted of approximately 1800 machines. Each machine had two 2GHz Intel Xeon processors with Hyper-Threading enabled, 4GB of memory, two 160GB IDE disks, and a gigabit Ethernet link. The machines were arranged in a two-level tree-shaped switched network with approximately 100-200Gbps of aggregate bandwidth available at the root. All of the machines were in the same hosting facility and therefore the roundtrip time between any pair of machines was less than a millisecond.

所有的程序都是在一个由大约1800台机器组成的集群上执行的。每台机器都有两台2GHz的Intel Xeon 启用超线程功能的处理器，4GB内存，2个USB接口。160GB IDE磁盘，以及千兆以太网链接。这些机器是 排列在一个两级树形交换网络中，根部约有100-200Gbps的总带宽。所有的 机器在同一主机设施中，因此任何一对机器之间的往返时间都小于一毫秒。

---

Out of the 4GB of memory, approximately 1-1.5GB was reserved by other tasks running on the cluster. The programs were executed on a weekend afternoon when the CPUs, disks, and network were mostly idle.

在4GB的内存中，大约有1-1.5GB被运行在集群上的其他任务所保留。这些程序是在一个周末的下午执行的，当时CPU、磁盘和网络大多处于闲置状态。

---

### **5.2 Grep**

The grep program scans through 1010 100-byte records, searching for a relatively rare three-character pattern (the pattern occurs in 92,337 records). The input is split into approximately 64MB pieces (M = 15000), and the entire output is placed in one file (R = 1).

grep程序扫描1010条100字节的记录，搜索一个比较罕见的三字符模式（该模式出现在92337条记录中）。输入的内容被分割成大约64MB的片段(M=15000)，整个输出被放在一个文件中(R=1)。

---

Figure 2 shows the progress of the computation over time. The Y-axis shows the rate at which the input data is scanned.  The rate gradually picks up as more machines are assigned to this MapReduce computation and peaks at over 30 GB/s when 1764 workers have been assigned.  As the map tasks finish, the rate starts dropping and hits zero about 80 seconds into the computation. The entire computation takes approximately 150 seconds from start to finish. This includes about a minute of startup overhead. The overhead is due to the propagation of the program to all worker machines and delays interacting with GFS to open the set of 1000 input files and to get the information needed for the locality optimization.

图2显示了计算进度随时间的变化。Y轴显示的是扫描输入数据的速度。 随着更多的机器被分配到这个MapReduce计算中，速率逐渐加快，当分配到1764个工人时，速率达到峰值，超过30GB/s。 随着地图任务的完成，速率开始下降，并达到零。在计算的80秒左右。整个计算从开始到结束大约需要150秒。这包括大约一分钟的启动开销。这个开销是由于程序传播到所有的工作机，以及与GFS交互的延迟，以打开1000个输入文件集，并获得定位优化所需的信息。

---

### **5.3 Sort**

The sort program sorts 1010 100-byte records (approximately 1 terabyte of data). This program is modeled after the TeraSort benchmark [12].

排序程序对1010条100字节的记录（大约1TB的数据）进行排序。这个程序是以TeraSort基准[12]为蓝本的。

---

The sorting program consists of less than 50 lines of user code. The final sorted output is written to a set of 2-way replicated GFS files (i.e., 2 terabytes are written as the output of the program).

排序程序由不到50行的用户代码组成。最终的排序输出被写入一组双向复制的GFS文件中（即写入2TB的程序输出）。

---

As before, the input data is split into 64MB pieces (*M* = 15000). We partition the sorted output into 4000 files (*R* = 4000). The partitioning function uses the initial bytes of the key to segregate it into one of pieces.

和之前一样，输入数据被分割成64MB的碎片（M=15000）。

我们将分类输出的数据分割成4000个文件（R=4000）。分区函数使用密钥的初始字节将其分离为其中的一个片段。

---

Our partitioning function for this benchmark has built-in knowledge of the distribution of keys. In a general sorting program, we would add a prepass MapReduce operation that would collect a sample of the keys and use the distribution of the sampled keys to compute splitpoints for the final sorting pass.

我们这个基准的分区函数内置了键的分布知识。在一般的排序程序中，我们会添加一个预通证MapReduce操作，该操作会收集密钥的样本，并使用采样密钥的分布来计算最终排序通证的分割点。

---

Figure 3 shows the progress of a normal execution of the sort program. The top-left graph shows the rate at which input is read. The rate peaks at about 13GB/s and dies off fairly quickly since all map tasks finish before 200 seconds have elapsed. Note that the input rate is less  than for grep. This is because the sort map tasks spend about half their
time and I/O bandwidth writing intermediate output to their local disks. The corresponding intermediate output for grep had negligible size.

图3显示了正常执行排序程序的进度。左上图显示了读取输入的速度。这个速度在13GB/s左右达到峰值，然后很快就消失了，因为所有的map任务在200秒之前就完成了。请注意，输入速率比grep要低。这是因为排序地图任务花了大约一半的时间来完成他们的工作的时间和I/O带宽将中间输出写入其本地磁盘。grep对应的中间输出的大小可以忽略不计。

---

A few things to note: the input rate is higher than the shuffle rate and the output rate because of our locality optimization; most data is read from a local disk and bypasses our relatively bandwidth constrained network. The shuffle rate is higher than the output rate because the output phase writes two copies of the sorted data (we make two replicas of the output for reliability and availability reasons). We write two replicas because that is the mechanism for reliability and availability provided by our underlying file system.  Network bandwidth requirements for writing data would be reduced if the underlying file system used erasure coding [15] rather than replication.

需要注意的是：输入率高于洗牌率和输出率是因为我们的定位优化；大部分数据是从本地磁盘读取的，绕过了我们带宽相对受限的网络。洗牌率比输出率高，是因为输出阶段写了两个排序数据的副本（出于可靠性和可用性的考虑，我们对输出做了两个副本）。我们写两个副本是因为这是我们底层文件系统提供的可靠性和可用性的机制。 如果底层文件系统使用擦除编码[15]而不是复制，那么写数据的网络带宽要求就会降低。。

The original article has further experiments that examine the effects of backup tasks and machine failures [8].

原文有进一步的实验，研究备份任务和机器故障的影响[8]。

---

### **6 Experience**

We wrote the first version of the MapReduce library in February of 2003 and made significant enhancements to it in August of 2003, including the locality optimization, dynamic load balancing of task execution across worker machines, etc. Since that time, we have been pleasantly surprised at how broadly applicable the MapReduce library
has been for the kinds of problems we work on. It has been used across a wide range of domains within Google, including:

我们在2003年2月编写了MapReduce库的第一个版本，并在2003年8月对其进行了重大改进，包括定位优化、跨工作机执行任务的动态负载均衡等。从那时起，我们对MapReduce库的广泛适用性感到非常惊喜。
一直以来，我们所研究的各类问题。它已经在谷歌内部的广泛域使用，包括。

---

- large-scale machine learning problems, 

- clustering problems for the Google News and Froogle products, 

- extracting data to produce reports of popular queries (e.g. Google
  Zeitgeist and Google Trends), 

- extracting properties of Web pages for new experiments and products (e.g. extraction of geographical locations from a large corpus of Web pages for localized search), 

- processing of satellite imagery data, 

- language model processing for statistical machine translation, and •

-  large-scale graph computations.

  

- 大型机器学习问题。

- 谷歌新闻和Froogle产品的集群问题。
- 提取数据，生成热门查询报告（如：Google）。
Zeitgeist和Google Trends）。) 
- 为新的实验和产品提取网页的属性（例如，从大量的语料库中提取地理位置）。本地化搜索的网页）。) 
- 卫星图像数据的处理； 
- 语言模型处理的统计机器翻译，以及 
- 大规模图计算。

---

Figure 4 shows the significant growth in the number of separate MapReduce programs checked into our primary source-code management system over time, from 0 in early 2003 to almost 900 in Septem - ber 2004, to about 4000 in March 2006. MapReduce has been so successful because it makes it possible to write a simple program and run it efficiently on a thousand machines in a half hour, greatly speeding up the development and prototyping cycle. Furthermore, it allows programmers who have no experience with distributed and/or parallel systems to exploit large amounts of resources easily.

图4显示了单独的数量的显著增长。随着时间的推移，MapReduce程序在我们的主要源码管理系统中的检查，从2003年初的0个到9月的近900个。ber 2004年，到2006年3月达到约4000人。MapReduce已经如此 成功的原因是，它使得编写一个简单的程序和 半小时内在千台机器上高效运行，大大加快了开发和原型设计周期。此外，它还可以 没有任何分布式和/或并行经验的程序员。系统，以便轻松利用大量资源。

---

At the end of each job, the MapReduce library logs statistics about the computational resources used by the job. In Table I, we show some statistics for a subset of MapReduce jobs run at Google in various months, highlighting the extent to which MapReduce has grown and become the de facto choice for nearly all data processing needs at Google

在每个作业结束时，MapReduce库会记录作业使用的计算资源的统计数据。在表一中，我们展示了不同月份在Google运行的MapReduce作业子集的一些统计数据，突出了MapReduce的发展程度，并成为Google几乎所有数据处理需求的事实上的选择。

---

### **6.1 Large-Scale Indexing**

One of our most significant uses of MapReduce to date has been a complete rewrite of the production indexing system that produces the data structures used for the Google Web search service. The indexing system takes as input a large set of documents that have been retrieved by our crawling system, stored as a set of GFS files. The raw contents for these documents are more than 20 terabytes of data. At the time we converted the indexing system to use MapReduce in 2003, it ran as a sequence of eight MapReduce operations. Since that time, because of the ease with which new phases can be added, many new phases have been added to the indexing system. Using MapReduce (instead of the ad-hoc distributed passes in the prior version of the indexing system) has provided several benefits.

迄今为止，我们对MapReduce最重要的使用之一是完全重写生产索引系统，该系统产生用于Google网络搜索服务的数据结构。该索引系统将我们的抓取系统检索到的大量文档作为输入，这些文档以一组GFS文件的形式存储。这些文件的原始内容是超过20TB的数据。在2003年我们将索引系统转换为使用MapReduce时，它是以8个MapReduce操作的序列运行的。从那时起，由于可以轻松地添加新的阶段，许多新的阶段被添加到索引系统中。使用MapReduce（而不是之前版本的索引系统中的临时分布式通证）提供了几个好处。

---

 The indexing code is simpler, smaller, and easier to understand because the code that deals with fault tolerance, distribution, and parallelization is hidden within the MapReduce library. For example, the size of one phase of the computation dropped from approximately 3800 lines of C++ code to approximately 700 lines when expressed using MapReduce.

 索引代码更简单、更小、更容易理解，因为处理容错、分布和并行化的代码都隐藏在MapReduce库中。例如，一个阶段的计算大小从大约3800行C++代码下降到使用MapReduce表达时大约700行。

---

The performance of the MapReduce library is good enough that we can keep conceptually unrelated computations separate instead of mixing them together to avoid extra passes over the data. This makes it easy to change the indexing process. For example, one change that took a few months to make in our old indexing system took only a
few days to implement in the new system.

MapReduce库的性能足够好，我们可以将概念上不相关的计算分开，而不是混合在一起，以避免在数据上进行额外的传递。这使得改变索引过程变得很容易。例如，在我们旧的索引系统中，一个需要几个月才能完成的改变，只需要一个
几天时间在新系统中实施。

The indexing process has become much easier to operate because most of the problems caused by machine failures, slow machines, and networking hiccups are dealt with automatically by the MapReduce library without operator intervention. Furthermore, it is easy to improve the performance of the indexing process by adding new machines to the indexing cluster.

索引过程的操作变得更加简单，因为大部分由机器故障、机器速度慢、网络故障引起的问题都会由MapReduce库自动处理，无需操作人员干预。此外，通过向索引集群添加新的机器，很容易提高索引过程的性能。



### **7 Related Work**

Many systems have provided restricted programming models and used the restrictions to parallelize the computation automatically.  For example, an associative function can be computed over all prefixes of an N element array in log N time on N processors using parallel prefix computations [6, 11, 14]. MapReduce can be considered a simplification and distillation of some of these models based on our experience with large real-world computations. More significantly, we provide a fault-tolerant implementation that scales to thousands of processors. In contrast, most of the parallel processing systems have only been implemented on smaller scales and leave the details of handling machine failures to the programmer.

许多系统都提供了限制性的编程模型，并使用了 的限制来自动并行化计算。 例如，一个关联函数可以在N个处理器上用对数N个时间对N个元素数组的所有前缀进行计算，使用并行前缀计算[6，11，14]。MapReduce可以认为是对以下内容的简化和提炼 其中的一些模型是基于我们在大型现实世界计算中的经验。更重要的是，我们提供了一个可扩展到数千个处理器的容错实现。相比之下，大多数并行处理系统只在较小的规模上实现，并将处理机器故障的细节留给了程序员。

---

Our locality optimization draws its inspiration from techniques such as active disks [13, 17], where computation is pushed into processing elements that are close to local disks, to reduce the amount of data sent across I/O subsystems or the network.

我们的区位性优化从主动磁盘[13,17]等技术中获得灵感，在这些技术中，计算被推送到靠近本地磁盘的处理元素中，以减少跨I/O子系统或网络发送的数据量。

---

The sorting facility that is a part of the MapReduce library is similar in operation to NOW-Sort [3]. Source machines (map workers) partition the data to be sorted and send it to one of R reduce workers. Each reduce worker sorts its data locally (in memory if possible). Of course NOW-Sort does not have the user-definable map and reduce functions that make our library widely applicable.

作为MapReduce库一部分的排序设施在操作上与NOW-Sort[3]类似。源机器（map worker）对要排序的数据进行分区，并将其发送给R reduce worker中的一个。每个reduce worker在本地（如果可能的话，在内存中）对其数据进行排序。当然NOW-Sort没有用户可定义的map和reduce函数，这使得我们的库得到了广泛的应用。

---

BAD-FS [5] and TACC [9] are two other systems that rely on re-execution as a mechanism for implementing fault tolerance. The original article has a more complete treatment of related work [8]

BAD-FS[5]和TACC[9]是另外两个依靠重执行作为实现容错机制的系统。原文中对相关工作有较为完整的处理[8]。

---

## **Conclusions**

The MapReduce programming model has been successfully used at Google for many different purposes. We attribute this success to several reasons. First, the model is easy to use, even for programmers without experience with parallel and distributed systems, since it hides the details of parallelization, fault tolerance, locality optimization, and load balancing. Second, a large variety of problems are easily expressible as MapReduce computations. For example, MapReduce is used for the generation of data for Google’s production Web search service, for sorting, data mining, machine learning, and many other systems. Third, we have developed an implementation of MapReduce that scales to large clusters of machines comprising thousands of machines. The implementation makes efficient use of these machine resources and therefore is suitable for use on many of the large computational problems encountered at Google.

MapReduce编程模型已经在Google成功地用于许多不同的目的。我们将这种成功归功于几个原因。首先，该模型很容易使用，即使是对没有并行和分布式系统经验的程序员也是如此，因为它隐藏了并行化、容错、局部优化和负载平衡的细节。其次，大量的问题都可以轻松地表达为MapReduce计算。例如，MapReduce被用于Google生产的Web搜索服务的数据生成，用于排序、数据挖掘、机器学习等许多系统。第三，我们开发了一个MapReduce的实现，可以扩展到由数千台机器组成的大型机器集群。该实现可以有效地利用这些机器资源，因此适合用于Google遇到的许多大型计算问题。

---

By restricting the programming model, we have made it easy to parallelize and distribute computations and to make such computations fault tolerant. Second, network bandwidth is a scarce resource. A number of optimizations in our system are therefore targeted at reducing the amount of data sent across the network: the locality optimization allows us to read data from local disks, and writing a single copy of the intermediate data to local disk saves network bandwidth. Third, redundant execution can be used to reduce the impact of slow machines, and to handle machine failures and data loss.

通过对编程模型的限制，我们使计算的并行化和分发变得容易，并且使这种计算 容错。其次，网络带宽是一种稀缺资源。A 因此，我们系统中的许多优化都是针对减少网络上发送的数据量：定位优化允许我们从本地磁盘中读取数据，并写入一份副本。的中间数据到本地磁盘，节省了网络带宽。第三： 冗余执行可以用来减少慢速的影响。机，并处理机器故障和数据丢失。

---

## References

1. Hadoop: Open source implementation of MapReduce. http://lucene. apache.org/hadoop/.
2. The Phoenix system for MapReduce programming. http:// csl.stanford. edu/~christos/sw/phoenix/.
3. Arpaci-Dusseau, A. C., Arpaci-Dusseau, R. H., Culler, D. E., Hellerstein, J. M., and Patterson, D. A. 1997. Highperformance sorting on networks of workstations. In Proceedings of the 1997 ACM SIGMOD International Conference on Management of Data. Tucson, AZ. 4 Barroso, L. A., Dean, J., and Urs Hölzle, U. 2003. Web search for a planet: The Google cluster architecture. IEEE Micro 23, 2, 22-28.
4. Bent, J., Thain, D., Arpaci-Dusseau, A. C., Arpaci-Dusseau, R. H., and Livny, M. 2004. Explicit control in a batch-aware distributed file system. In Proceedings of the 1st USENIX Symposium on Networked Systems Design and Implementation (NSDI).
5. Blelloch, G. E. 1989. Scans as primitive parallel operations. IEEE Trans. Comput. C-38, 11.
6. Chu, C.-T., Kim, S. K., Lin, Y. A., Yu, Y., Bradski, G., Ng, A., and Olukotun, K. 2006. Map-Reduce for machine learning on multicore. In Proceedings of Neural Information Processing Systems Conference (NIPS). Vancouver, Canada.
7. Dean, J. and Ghemawat, S. 2004. MapReduce: Simplified data processing on large clusters. In Proceedings of Operating Systems Design and Implementation (OSDI). San Francisco, CA. 137-150.
9. Fox, A., Gribble, S. D., Chawathe, Y., Brewer, E. A., and Gauthier, P.
9. Cluster-based scalable network services. In Proceedings of the 16th ACM Symposium on Operating System Principles. Saint-Malo, France. 78-91.
10. Ghemawat, S., Gobioff, H., and Leung, S.-T. 2003. The Google file system. In 19th Symposium on Operating Systems Principles. Lake George, NY. 29-43.
11. Gorlatch, S. 1996. Systematic efficient parallelization of scan and other list homomorphisms. In L. Bouge, P. Fraigniaud, A. Mignotte, and Y. Robert, Eds. Euro-Par’96. Parallel Processing, Lecture Notes in Computer Science, vol. 1124. Springer-Verlag. 401-408
12. Gray, J. Sort benchmark home page. http:// research. microsoft. com/barc/ SortBenchmark/.
13. Huston, L., Sukthankar, R., Wickremesinghe, R., Satyanarayanan, M., Ganger, G. R., Riedel, E., and Ailamaki, A. 2004. Diamond: A storage architecture for early discard in interactive search. In Proceed ings of the 2004 USENIX File and Storage Technologies FAST Conference.
14. Ladner, R. E., and Fischer, M. J. 1980. Parallel prefix computation. JACM 27, 4. 831-838.
15. Rabin, M. O. 1989. Efficient dispersal of information for security, load balancing and fault tolerance. JACM 36, 2. 335-348.
16. Ranger, C., Raghuraman, R., Penmetsa, A., Bradski, G., and Kozyrakis, C. 2007. Evaluating mapreduce for multi-core and multiprocessor systems. In Proceedings of 13th International Symposium on High-Performance Computer Architecture (HPCA). Phoenix, AZ.
17. Riedel, E., Faloutsos, C., Gibson, G. A., and Nagle, D. Active disks for large-scale data processing. IEEE Computer. 68-74.

