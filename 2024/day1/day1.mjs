import fs from "node:fs";

const sample1 = fs.readFileSync("sample1.txt", { encoding: "utf-8" });

console.log(part1(sample1));

/**
 *
 * @param {String} fileContent
 */
function part1(fileContent) {
  const lines = fileContent.trim().split("\r\n");
  const leftDists = [];
  const rightDists = [];

  lines.map((l) => {
    const [left, right] = l.split("   ");
    leftDists.push(left);
    rightDists.push(right);
  });

  leftDists.sort((a, b) => a - b);
  rightDists.sort((a, b) => a - b);

  let distSum = 0;

  for (let i = 0; i < leftDists.length; i++) {
    const leftDist = leftDists[i];
    const rightDist = rightDists[i];

    distSum += Math.abs(leftDist - rightDist);
  }

  return distSum;
}
