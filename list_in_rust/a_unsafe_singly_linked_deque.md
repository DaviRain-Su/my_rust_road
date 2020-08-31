# A unsafe singly linked Queue

reference-counted interior mutability stuff 

Rc and RefCell can be great for handling simple cases, but they can get unwiely.

a singly linked list and implement a singly linked queue to dip our 
toes into raw pointer and unsafe rust.

# lay out 

### for stack 

input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

stack push X:
[Some(ptr)] -> (X, Some(ptr)) -> (A, Some(ptr)) -> (B, None)

stack pop:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

### for queue

#### queue push 

input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

flipped push X:
[Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

#### queue pop

input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

flipped pop:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)



