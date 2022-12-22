import fs from "fs";
import path from "path";

const evaluateFromRoot = (value, obj) => {
  // Leaf, just return the number
  if (typeof value === "number") {
    return value;
  }

  const [head, operator, tail] = value;

  // Recurse down to the leaves
  const rHead = evaluateFromRoot(obj[head], obj);
  const rTail = evaluateFromRoot(obj[tail], obj);

  return {
    "+": rHead + rTail,
    "-": rHead - rTail,
    "*": rHead * rTail,
    "/": rHead / rTail,
  }[operator];
};

const convert = (node) => {
  const [key, val] = node.split(": ");

  // If maybeInt !== maybeInt, it's NaN, so an expression rather than an int
  const maybeInt = parseInt(val, 10);
  const value = maybeInt === maybeInt ? maybeInt : val.split(" ");

  return { key, value };
};

export function _getTotal(nodes) {
  const obj = nodes.reduce((acc, node) => {
    const { key, value } = convert(node);
    acc[key] = value;
    return acc;
  }, {});

  return evaluateFromRoot(obj.root, obj);
}

export function main() {
  const nodes = fs
    .readFileSync(new URL("input.txt", import.meta.url), "utf-8")
    .trim()
    .split("\n");

  console.log("converted:", _getTotal(nodes));
}
