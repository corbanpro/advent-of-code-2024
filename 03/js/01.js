import fs from "fs";

const mulInput = fs.readFileSync("03/assets/mul.txt").toString();

const regex = /mul\((\d+),(\d+)\)/g;

const matches = mulInput.match(regex);

let sum = 0;

for (let match of matches) {
  const matchRegex = /(\d+)/g;
  const numbers = match.match(matchRegex);
  sum += Number(numbers[0]) * Number(numbers[1]);
}
console.log(sum);
