# Ownership

Contrasts to explicit allocation and freeing of memory.

## The Stack and the Heap

Both are parts of memory available to a program, but they are structured differently.

### The Stack

Stores values in the order it receives them and and removes values in the opposite order. LIFO.

Pushing onto the stack - adding data
Poping off the stack - removing data

#### Speed

 - The stack never has to search for memory when adding or removing, it's always "on top."
 - All data on the stack must take up a known fixed size.

### The Heap

Useful for storing data whose size is not known during compile time or might change.

The heap is less organized. We request some space, the OS finds space that's large enough, marks it as in use then returns a pointer to the start of the space.

Allocating on the heap - The process described above

## Function Calls

Values passed into a function are stored on the stack. The functions local variables are also stored on the stack.
When the function is over those variables get popped off the stack.

## What ownership tries to address

 - "Keeping track of what parts of code are using what data on the heap"
 - "minimizing the amount of duplicate data on the heap"
 - "cleaning up unused data on the heap so we don’t run out of space"

