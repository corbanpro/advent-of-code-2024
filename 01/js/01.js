import fs from "fs";

const locations = fs.readFileSync("../assets/locations.txt").toString();

const rows = locations.split("\n");

let left = rows.map((row) => row.split(/\s+/)[0]).filter((r) => r);
let right = rows.map((row) => row.split(/\s+/)[1]).filter((r) => r);

left.sort((a, b) => a - b);
right.sort((a, b) => a - b);

let distance = 0;
for (let i = 0; i < left.length; i++) {
  distance += Math.abs(left[i] - right[i]);
}
console.log(distance);
