import fs from 'fs/promises';
import path from 'path';

const sum = (a, c) => a + parseInt(c, 10);

export async function main() {
  const file = new URL('input.txt', import.meta.url);
  const data = await fs.readFile(file, 'utf-8');
  const calories = data
    .split('\n\n')
    .map((line) => line.trim().split('\n').reduce(sum, 0))
    .sort((a, b) => (a < b ? 1 : -1))
    .slice(0, 3)
    .reduce(sum, 0);

  // Part 1
  // console.log('Math.max(...calories)', Math.max(...calories));
  console.log('calories', calories);
}
