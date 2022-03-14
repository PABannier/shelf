# A lightweight, functional distributed file system written in Rust

A lightweight, ready-to-use distributed file system written in Rust.

3 key components:

- Master server: receives and executes commands
- Volume server: store the actual files
- Filer store (here, Redis): key-value store used for indexing files

Under the hood, like ![minikeyvalue](github.com/geohot/minikeyvalue), we're
using Nginx stock, and Redis as the filer store.

We want this file system to be as simple as possible, by supporting the most
frequently used commands, no less, no more.

When adding or removing a volume, rebalancing is carried out to ensure optimal
performance between the storage units.

## API

As clients, we want to:

- GET a key
  That returns a value or a file from the volume server or the k-v store

- PUT a key
  That pushes a value or a file on a volume server.
  If already exists, update the value or the key.

- DELETE a key
  If key exists, deletes the key or the file.

As system administrator, we want to:

- Link a database (here, Redis URI e.g.: http://localhost:6379)

- Select which ports we are listening on (e.g: 3000)

- Select the number of replicas (amount of replicas to make of the data)

- Select the number of subvolumes (amount of disks per machine)

- Select the number of volumes used for storage (as many volume server as number of machines)

- Rebalance the storage volumes (when adding one volume or removing one)

- Rebuild the index (Redis, key-value storage)

## Benchmarks
