# Rust-Recommender

Rust-Recommender is about code recommendation. 
Now we are trying to establish a LSP server to provide service.
Eventually, we can recommend right Rust code segment to replace error code segment.

## How does it work
*Rust-Recommender* is to leverage the Language Server Protocol to recommend similar errors and bug fixes to help programmers debug. We built a knowledge base by collecting various compilation errors discussed on Stackoverflow, Reddit, and other related websites. Meanwhile, we will build an IDE plugin to use the database and make recommendations to developers when they encounter compilation errors. 


We divide the workflow into two parts.
First part is the communication between IDE and backend.
We do compilicate calculaition in the backend.
![First](https://github.com/Artisan-Lab/Rust-Recommender/image/howwork1.png)

Then we see how the language server work.

![Second](https://github.com/Artisan-Lab/Rust-Recommender/image/howwork2.png)

## What it will look like?

![Little example](https://github.com/Artisan-Lab/Rust-Recommender/image/looklike.png)


## Usage
```shell
Milestone
```

