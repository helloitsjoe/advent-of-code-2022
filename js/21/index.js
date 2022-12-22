import fs from "fs";
import path from "path";

/**
 * Recursively evaluates from the root
 * @returns {number} Total in tree
 */
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

/**
 * Converts strings into nodes with key/value
 * @returns {Object} key/value
 */
const convert = (node) => {
  const [key, val] = node.split(": ");

  // If maybeInt !== maybeInt, it's NaN, so an expression rather than an int
  const maybeInt = parseInt(val, 10);
  const value = maybeInt === maybeInt ? maybeInt : val.split(" ");

  return { key, value };
};

/**
 * Creates an object representation of the nodes, export for unit tests only
 * @returns {number} Total
 */
export function _getTotal(nodes) {
  const obj = nodes.reduce((acc, node) => {
    const { key, value } = convert(node);
    acc[key] = value;
    return acc;
  }, {});

  return evaluateFromRoot(obj.root, obj);
}

/**
 * Main function called by index.js
 */
export function main() {
  const nodes = fs
    .readFileSync(new URL("input.txt", import.meta.url), "utf-8")
    .trim()
    .split("\n");

  console.log("converted:", _getTotal(nodes));
}
