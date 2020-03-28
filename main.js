import init, { run_app } from "./pkg/yew_tutorial.js";
async function main() {
  await init("/pkg/yew_tutorial_bg.wasm");
  run_app();
}
main();
