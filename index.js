import { pre_validate, is_valid_event } from "./pkg";
import { createRuntime } from "./runtime";

createRuntime();
pre_validate();
console.log("init pre_validate..");

const test_event = {
  content: `I recommend looking into nspawn - it's a systemd enabled service that runs LXC containers

  here is an example of a deployment script i built most of:
  
  GitHub - relaytools/relay-tools-images: build and deploy repository for relay tools(build and deploy repository for relay tools. ..)
  
  it's a bit more manual than docker but you aren't forced into the use of aufs or whatever overlay filesystem so it's got a bit less overhead at that level
  
  performance is pretty much near the same as running the server not in a container, it mostly only controls access to kernel resources via namespaces`,
  created_at: 1704354395,
  id: "a4e602e7ebb85bc3d3eae64476db5e2987d8370fc18f2b467b5c4c71fa8671da",
  kind: 1,
  pubkey: "8fb140b4e8ddef97ce4b821d247278a1a4353362623f64021484b372f948000c",
  sig: "cd986256fbdcc32b8064dd09bb2efc57e1d06914b8c4721b0524d35d70d59ccb471ea2df64c9c1cf47bb4b188c25718dbd56ce110073e563b326f0463b291fe1",
  tags: [
    ["e", "8a131cb2d6b0e92744cbb786edcf4ab39047686b78d04bbfb4d6ad0d23efa603"],
    ["p", "a8171781fd9e90ede3ea44ddca5d3abf828fe8eedeb0f3abb0dd3e563562e1fc"],
  ],
  seen: ["wss://relay.damus.io/", "wss://relay.nostr.band/"],
  timestamp: 1704354462068,
};
const test_event2 = {
  content: `What do I need to open PDF files?,,You need Adobe Acrobat Reader:  http://www.adobe.com/products/acrobat/r`,
  created_at: 1704354395,
  id: "a4e602e7ebb85bc3d3eae64476db5e2987d8370fc18f2b467b5c4c71fa8671da",
  kind: 1,
  pubkey: "8fb140b4e8ddef97ce4b821d247278a1a4353362623f64021484b372f948000c",
  sig: "cd986256fbdcc32b8064dd09bb2efc57e1d06914b8c4721b0524d35d70d59ccb471ea2df64c9c1cf47bb4b188c25718dbd56ce110073e563b326f0463b291fe1",
  tags: [
    ["e", "8a131cb2d6b0e92744cbb786edcf4ab39047686b78d04bbfb4d6ad0d23efa603"],
    ["p", "a8171781fd9e90ede3ea44ddca5d3abf828fe8eedeb0f3abb0dd3e563562e1fc"],
  ],
  seen: ["wss://relay.damus.io/", "wss://relay.nostr.band/"],
  timestamp: 1704354462068,
};
const test_event3 = {
  content:
    "Noticed he said CONTENT.  Content is information aside from the website design and layout.\n\nContent on a website is best unique, as opposed to syndicated (taken from someone else).\n\nIf your site is uniquely written, that's good.  If it's well written, that's great.\n\nIf your site is writing about an interesting topic, uniquely written by you, and well-written, that's awesome -- That's the premisis for creating content for a good website.\n\nAlso, content can come in forms of raw data -- Like, if you wanted to make a travel site -- you can list the prices it takes to fly from Chicago to Pittsburg.  Or you can have a currency converter.\n\nAnother thing you can incorporate is a chatroom or message board for users to discuss the content and post their suggestions.",
};
const test_event4 = {
  content:
    "What is the US market size ($)for medical diagnostic equipment for 2006?",
};

const result = is_valid_event(test_event);
console.log(test_event.content, result);

const result2 = is_valid_event(test_event2);
console.log(test_event2.content, result2);

const result3 = is_valid_event(test_event3);
console.log(test_event3.content,result3);

const result4 = is_valid_event(test_event4);
console.log(test_event4.content, result4);
