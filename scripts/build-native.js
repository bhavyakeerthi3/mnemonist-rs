'use strict';

const fs = require('fs');
const path = require('path');
const {spawnSync} = require('child_process');

const root = path.resolve(__dirname, '..');

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: root,
    stdio: 'inherit',
    shell: process.platform === 'win32'
  });

  if (result.status !== 0) {
    process.exit(result.status || 1);
  }
}

// Windows hosts may use either Rust's MSVC target or a compatible x86_64
// MinGW toolchain. Prefer a known 64-bit MinGW installation when present;
// this avoids accidentally selecting an obsolete 32-bit C:\\MinGW first on PATH.
const mingwBins = [
  process.env.MNEMONIST_MINGW_BIN,
  'C:\\msys64\\ucrt64\\bin',
  'C:\\mingw64\\bin',
  'C:\\mingw64-winlibs\\mingw64\\bin'
].filter(candidate => candidate && fs.existsSync(path.join(candidate, 'dlltool.exe')));

if (process.platform === 'win32' && mingwBins.length > 0) {
  process.env.PATH = `${mingwBins[0]}${path.delimiter}${process.env.PATH}`;
}

const toolchain = process.env.MNEMONIST_RUST_TOOLCHAIN ||
  (process.platform === 'win32' && mingwBins.length > 0 ? 'stable-x86_64-pc-windows-gnu' : undefined);
const cargoArgs = process.platform === 'win32' && toolchain ?
  [`+${toolchain}`, 'build', '--release', '--no-default-features', '-Fnodejs', '--lib'] :
  process.platform === 'win32' ?
    ['+stable-x86_64-pc-windows-msvc', 'build', '--release', '--no-default-features', '-Fnodejs', '--lib'] :
    ['build', '--release', '--no-default-features', '-Fnodejs', '--lib'];

run('cargo', cargoArgs);

const release = path.join(root, 'target', 'release');
// Order candidates by the current platform first so a stale artifact left
// over from a different OS (e.g. a committed .dll next to a freshly built
// .so) is never picked over the one that actually matches this host.
const byPlatform = {
  win32: [path.join(release, 'mnemonist.dll')],
  darwin: [path.join(release, 'libmnemonist.dylib')],
  linux: [path.join(release, 'libmnemonist.so')]
};
const rest = [
  path.join(release, 'mnemonist.dll'),
  path.join(release, 'libmnemonist.so'),
  path.join(release, 'libmnemonist.dylib')
];
const sourceCandidates = [
  ...(byPlatform[process.platform] || []),
  ...rest
];
const source = sourceCandidates.find(candidate => fs.existsSync(candidate));

if (!source) {
  console.error('Could not find compiled native library in target/release.');
  process.exit(1);
}

const target = path.join(root, 'mnemonist.node');
fs.copyFileSync(source, target);
console.log(`Wrote ${path.relative(root, target)} from ${path.relative(root, source)}`);
