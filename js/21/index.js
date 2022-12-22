import fs from "fs";
import path from "path";

const evaluate = (value, obj) => {
  if (typeof value === "number") {
    return value;
  }

  const [head, operator, tail] = value;

  const rHead = evaluate(obj[head], obj);
  const rTail = evaluate(obj[tail], obj);

  if (operator === "+") return rHead + rTail;
  if (operator === "-") return rHead - rTail;
  if (operator === "*") return rHead * rTail;
  if (operator === "/") return rHead / rTail;
};

const convert = (node) => {
  const [id, val] = node.split(": ");

  const maybeInt = parseInt(val);
  // NaN check - if maybeInt !== maybeInt, it's NaN, so an expression
  const value = maybeInt === maybeInt ? maybeInt : val.split(" ");

  return {
    id,
    value,
  };
};

export function convertNodes(nodes) {
  const obj = {};

  for (const node of nodes) {
    const converted = convert(node);
    const { id, value } = converted;
    obj[id] = value;
  }

  let total = evaluate(obj.root, obj);
  return total;
}

export function main() {
  const file = fs.readFileSync(new URL("input.txt", import.meta.url), "utf-8");

  // console.log("file", file);
  const nodes = file.trim().split("\n");

  console.log("converted:", convertNodes(nodes));
}
