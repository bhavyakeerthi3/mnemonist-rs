'use strict';

const path = require('path');

let failures = 0;
const stack = [];

global.describe = function(name, fn) {
  stack.push(name);
  try {
    fn();
  }
  finally {
    stack.pop();
  }
};

global.it = function(name, fn) {
  const label = [...stack, name].join(' > ');
  try {
    fn();
    console.log(`ok ${label}`);
  }
  catch (error) {
    failures++;
    console.error(`not ok ${label}`);
    console.error(error && error.stack || error);
  }
};

global.it.skip = function() {};

for (const file of process.argv.slice(2)) {
  require(path.resolve(file));
}

if (failures) {
  console.error(`${failures} original adapter test(s) failed.`);
  process.exit(1);
}

console.log('Original adapter tests passed.');
