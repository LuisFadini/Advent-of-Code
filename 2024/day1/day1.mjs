import fs from "node:fs";

const input = fs.readFileSync("input.txt", { encoding: "utf-8" });

console.log(part2(input));

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

/**
 *
 * @param {String} fileContent
 */
function part2(fileContent) {
  const lines = fileContent.trim().split("\r\n");
  const leftDists = [];
  const rightDists = [];

  lines.map((l) => {
    const [left, right] = l.split("   ");
    leftDists.push(left);
    rightDists.push(right);
  });

  let lookup = new Map();

  let distSum = 0;

  for (let i = 0; i < leftDists.length; i++) {
    const leftDist = leftDists[i];
    if (lookup.has(leftDist)) {
      distSum += lookup.get(leftDist);
      continue;
    }

    if (!rightDists.includes(leftDist)) {
      continue;
    }

    let similarityScore =
      rightDists.filter((f) => f === leftDist).length * leftDist;
    lookup.set(leftDist, similarityScore);
    distSum += similarityScore;
  }

  return distSum;
}
