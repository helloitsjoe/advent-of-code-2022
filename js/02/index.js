import fs from 'fs/promises';
import path from 'path';

const sum = (a, c) => a + parseInt(c, 10);

const computeScore = (input) => {
  const [a, b] = input.split(' ');
  const values = {
    A: 1,
    B: 2,
    C: 3,
    X: 1,
    Y: 2,
    Z: 3,
  };

  const valA = values[a];
  const valB = values[b];

  let score = valB;
  if (valA === valB) {
    score += 3;
  } else if (valB === valA + 1 || valB === valA - 2) {
    score += 6;
  }

  return score;
};

export async function main() {
  const file = new URL('input.txt', import.meta.url);
  const data = await fs.readFile(file, 'utf-8');

  const total = data.trim().split('\n').map(computeScore).reduce(sum, 0);

  console.log('total', total);
}
