import fs from "fs";

const locations = fs.readFileSync("../assets/locations.txt").toString();

const rows = locations.split("\n");

let left = rows.map((row) => row.split(/\s+/)[0]).filter((r) => r);
let right = rows.map((row) => row.split(/\s+/)[1]).filter((r) => r);

const ids = {};

for (let id of left) {
  ids[id] = 0;
}

for (let id of right) {
  if (ids[id] !== undefined) {
    ids[id] += 1;
  }
}

let similarityScore = 0;
for (let id in ids) {
  similarityScore += ids[id] * id;
}

console.log(similarityScore);
