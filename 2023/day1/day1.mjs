import fs from "node:fs";

const sample1 = fs.readFileSync("sample1.txt", { encoding: "utf-8" });

console.log(part1(sample1));

/**
 *
 * @param {String} file
 */
function part1(file) {
  const lines = file.split("\r\n");
  let sum = 0;

  lines.forEach((l) => {
    let firstNum = -1;
    let lastNum = -1;

    l.split("")
      .map((c) => Number(c))
      .filter((c) => !Number.isNaN(c))
      .forEach((n) => {
        if (firstNum === -1) firstNum = n;
        lastNum = n;

        console.log(firstNum);
        console.log(lastNum);
      });

    sum += 10 * firstNum + lastNum;
  });

  return sum;
}
