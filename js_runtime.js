const algo_event = {
  content: "",
  created_at: 1704354395,
  id: "a4e602e7ebb85bc3d3eae64476db5e2987d8370fc18f2b467b5c4c71fa8671da",
  kind: 32024,
  pubkey: "8fb140b4e8ddef97ce4b821d247278a1a4353362623f64021484b372f948000c",
  sig: "cd986256fbdcc32b8064dd09bb2efc57e1d06914b8c4721b0524d35d70d59ccb471ea2df64c9c1cf47bb4b188c25718dbd56ce110073e563b326f0463b291fe1",
  tags: [
    ["spams", "3", "1", "0", "5", "2"],
    ["hams", "0", "1", "3", "0", "1"],
    ["tokens", "docker", "suggest", "faggot", "linux", "computer"],
  ],
  seen: ["wss://relay.damus.io/", "wss://relay.nostr.band/"],
  timestamp: 1704354462068,
};

// Define the selfEvent function
function selfEvent() {
  return algo_event;
}

// Expose the selfEvent function to the WebAssembly module
export { selfEvent };
