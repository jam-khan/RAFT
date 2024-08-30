# Distributed Key-Value Datastore with Raft Consensus

This project implements a simple, distributed, replicated key-value datastore using the Raft consensus protocol.

## Project Description

This project aims to build a distributed key-value datastore that supports strong consistency guarantees. The datastore will be implemented as a multi-process system using the Raft consensus protocol to maintain consensus among replicas.

### Key Features

* **Distributed:** The datastore is distributed across multiple nodes, enabling scalability and fault tolerance.
* **Replicated:** Data is replicated across multiple nodes to ensure high availability and data durability.
* **Strong Consistency:** The system guarantees that all clients see the same, up-to-date data, even in the presence of failures.
* **Raft Consensus:** The Raft protocol is used to achieve consensus among replicas, ensuring data consistency and leader election.

## High-Level Approach

The project implements the Raft consensus algorithm by structuring the Replica class to handle various responsibilities:

* **Leader Election:** Replicas initiate elections to choose a new leader when the current leader fails or the system starts up.
* **Log Replication:** The leader replicates logs to followers to maintain a consistent state across all replicas.
* **Client Interaction:** Replicas handle `put` and `get` requests from clients, redirecting requests to the leader if necessary.
* **Heartbeat Mechanism:** Leaders periodically send heartbeat messages to maintain their authority and prevent unnecessary elections.

## Challenges Faced

* **Leader Election Stability:** Ensuring that only one leader is elected per term and avoiding frequent elections posed a significant challenge. This involved debugging election timeouts, managing vote requests and responses, and ensuring proper handling of conflicting elections.
* **Log Consistency:** Implementing robust checks before committing log entries and applying them to the state machine only after ensuring replication on a majority of servers was crucial. This required careful handling of `AppendEntries` related calls.

## Good Design Features

* **Modular Design:** The codebase is structured into well-defined functions and methods that handle specific tasks like voting, appending entries, and responding to client requests. This approach simplifies understanding, maintenance, and debugging.
* **Robust Error Handling:** The system is designed to handle errors gracefully, ensuring service continuity even during network issues or node failures.
* **State Machine Safety:** The state machine (key-value store) is only updated upon commitment of entries, preventing stale or incorrect reads and ensuring data integrity.

## Testing Strategy

* **Performance Measurement:** The performance test analysis provided at the end of the simulation was used to improve latency and optimize message communication between replicas.
* **Edge Cases:** Special attention was given to handling edge cases such as leadership changes during client requests and simultaneous leader elections.

## Future Improvements

* **Performance Optimization:** Further optimization of message communication and data replication can improve performance.
* **Fault Tolerance Enhancements:** Implementing mechanisms to handle more complex failure scenarios, such as network partitions, can enhance fault tolerance.
* **Scalability Improvements:** Exploring techniques for scaling the system to handle a larger number of nodes and clients can improve scalability.

## How to Run

To run the project, execute the following command in the project directory:

```bash
./run all