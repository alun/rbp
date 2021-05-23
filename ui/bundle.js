import init, { run_app } from './pkg/ui.js';
async function main() {
  await init('/pkg/ui_bg.wasm');
  run_app();
}
main();