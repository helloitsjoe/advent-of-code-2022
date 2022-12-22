import { it, describe, expect } from "vitest";
import { _getTotal } from "./index";

describe("getTotal", () => {
  it.each([
    [["root: a + b", "a: 2", "b: 2"], 4],
    [["root: a + b", "a: 2", "b: c + d", "c: 3", "d: 1"], 6],
    [["root: a + b", "a: 2", "b: c - d", "c: 3", "d: 1"], 4],
    [["root: a + b", "a: 2", "b: c * d", "c: 3", "d: 1"], 5],
    [["root: a + b", "a: 2", "b: c / d", "c: 3", "d: 1"], 5],
    [
      ["root: a + b", "a: e + f", "b: c + d", "c: 3", "d: 1", "e: 4", "f: 5"],
      13,
    ],
    [
      [
        "root: pppw + sjmn",
        "dbpl: 5",
        "cczh: sllz + lgvd",
        "zczc: 2",
        "ptdq: humn - dvpt",
        "dvpt: 3",
        "lfqf: 4",
        "humn: 5",
        "ljgn: 2",
        "sjmn: drzm * dbpl",
        "sllz: 4",
        "pppw: cczh / lfqf",
        "lgvd: ljgn * ptdq",
        "drzm: hmdt - zczc",
        "hmdt: 32",
      ],
      152,
    ],
  ])("simple", (nodes, answer) => {
    console.log("nodes", nodes);
    expect(_getTotal(nodes)).toBe(answer);
  });
});
