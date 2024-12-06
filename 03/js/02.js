import fs from "fs";

const rawInstructions = fs.readFileSync("03/assets/mul.txt").toString();

let filteredInstructions = "";
let activeReadInstructions = true;

for (let i = 0; i < rawInstructions.length; i++) {
  if (rawInstructions.slice(i - 7, i) === "don't()") {
    activeReadInstructions = false;
    filteredInstructions += "\n\n";
  } else if (rawInstructions.slice(i, i + 4) === "do()") {
    activeReadInstructions = true;
  }
  if (activeReadInstructions) {
    filteredInstructions += rawInstructions[i];
  }
}

const mulRegex = /mul\((\d+),(\d+)\)/g;

const mulCommands = filteredInstructions.match(mulRegex);

let productSum = 0;

for (let mulCommand of mulCommands) {
  const numberRegex = /(\d+)/g;
  const factors = mulCommand.match(numberRegex);
  productSum += Number(factors[0]) * Number(factors[1]);
}

console.log(productSum);
