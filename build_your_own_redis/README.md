# Build your own Redis with C/C++

## Chapter 1 - Introduction
This book help to practice 2 fundamental skills:
1. Network programming
2. Data structures

### What is Redis?
Just the most popular in-memory key-value store.  
A cache server is a `map[string]string` over a network. In Redis, the `value` can be  
and data structure:
- Hash
- List
- Sorted Set
This is why Redis is called a `data structure server`

## Chapter 2 - Socket programming
### From black boxes to code
What to learn:
1. TCP byte streams and protocols
	- TCP Deals in streams of bytes, that's it. It's the application's protocol Job to make sense of them.
2. Data serialization
	- **Serialization**: Mapping object to bytes
	- **Desrialization**: Mapping bytes to object
3. Concurrent programming
	- Most modern software use `event-based` concurrency models with `event-loops`

## Chapter 3 - TCP Server and client
### Endianess
There are 2 ways to store integers in memory
- **Little-endian**: The least significant byte comes first
- **Big-endian**: The most significant byte comes first (often called `network byte order`)

## Chapter 4 - Request-Response protocol
We need to create to create an application protocol to read the byte stream, and it has  
2 levels of structures:
1. High-level structure to split a byte stream into messages
2. The structure itself within a message, the deserialization/marshaling.
