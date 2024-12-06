import fs from "fs";

const data = fs.readFileSync("../assets/levels.txt").toString();
const rows = data.split("\n").filter((row) => row.length > 1);

let numSafe = 0;
for (let rawReadings of rows) {
  const readings = rawReadings.split(" ").map((num) => Number(num));
  const isIncreasing = readings[0] < readings[1];
  let isSafe = true;

  for (let i = 0; i < readings.length - 1; i++) {
    let diff = readings[i + 1] - readings[i];
    if (!isIncreasing) {
      diff = -diff;
    }
    if (![1, 2, 3].includes(diff)) {
      isSafe = false;
      break;
    }
  }
  if (isSafe) {
    numSafe += 1;
  }
}
console.log(numSafe);
