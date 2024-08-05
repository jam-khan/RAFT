enum RPC {
    AppendEntriesRequest,
    AppendEntriesResponse,
    VoteRequest,
    VoteResponse
}

enum Server {
    Leader,
    Candidate,
    Follower
}

fn main() {
    println!("Hello, world!");
}
