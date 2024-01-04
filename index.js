import { pre_validate, is_valid_event,rating, add_to_token_table, update_spams, update_hams, initSync } from './pkg';

//initSync()
const event = {"content":"Good suggestion docker","created_at":1704354395,"id":"a4e602e7ebb85bc3d3eae64476db5e2987d8370fc18f2b467b5c4c71fa8671da","kind":1,"pubkey":"8fb140b4e8ddef97ce4b821d247278a1a4353362623f64021484b372f948000c","sig":"cd986256fbdcc32b8064dd09bb2efc57e1d06914b8c4721b0524d35d70d59ccb471ea2df64c9c1cf47bb4b188c25718dbd56ce110073e563b326f0463b291fe1","tags":[["e","8a131cb2d6b0e92744cbb786edcf4ab39047686b78d04bbfb4d6ad0d23efa603"],["p","a8171781fd9e90ede3ea44ddca5d3abf828fe8eedeb0f3abb0dd3e563562e1fc"]],"seen":["wss://relay.damus.io/","wss://relay.nostr.band/"],"timestamp":1704354462068}
pre_validate();
const result = is_valid_event(event);
console.log(result);
/*
add_to_token_table("docker", 0n)
add_to_token_table("suggest", 1n)
add_to_token_table("faggot", 2n)

update_spams(0, 3)
update_spams(1, 1)
update_spams(2, 0)

update_hams(0, 0)
update_hams(1, 1)
update_hams(2, 3)

const result = rating(event.content)
console.log(result); // Verify the result
*/

