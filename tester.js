const fs = require('fs');
const path = require('path');
const { spawnSync } = require('child_process');

const testFolder = './test_data';

const stagesToRun = process.argv.slice(2);
const files = fs.readdirSync(testFolder)
  .filter(f => {
    const match = f.match(/^test_(\d{2})_/);
    return match && stagesToRun.includes(String(parseInt(match[1], 10)));
  })
  .sort();

let passed = 0;

for (let i = 0; i < files.length; i += 4) {
  const inputFile = path.join(testFolder, files[i]);
  const outputFile = path.join(testFolder, files[i + 1]);

  const input = fs.readFileSync(inputFile, 'utf8');
  const expectedOutput = fs.readFileSync(outputFile, 'utf8').trim();

  const result = spawnSync('cargo', ['run'], {
    input: input,
    encoding: 'utf8',
    shell: true,
  });

  const actualOutput = result.stdout.trim();

  if (actualOutput === expectedOutput) {
    console.log(`✅ Test ${files[i]} passed`);
    passed++;
  } else {
    console.log(`❌ Test ${files[i]} failed`);
    console.log(`Expected:\n${expectedOutput}`);
    console.log(`Got:\n${actualOutput}`);
  }
}

console.log(`\n${passed}/${files.length / 4} tests passed`);
