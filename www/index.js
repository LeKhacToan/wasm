import * as wasm from "wasm-game-of-life";

let a = [];
for (let i = 0; i < 100000000; i++) {
  a.push(Math.floor(Math.random() * 100000000) + 1);
}

const max_js = (array) => {
  let a = array[0];
  for (let i = 0; i < array.length; i++) {
    if (array[i] > a) {
      a = array[i];
    }
  }
  return a;
};

const fibonacci = (n) => {
  if (n === 0) return 1;
  if (n === 1) return 1;

  return fibonacci(n - 1) + fibonacci(n - 2);
};

let start = performance.now();
console.log(wasm.max(a));
console.log(performance.now() - start);

start = performance.now();
console.log(max_js(a));
console.log(performance.now() - start);

start = performance.now();
console.log(wasm.fibonacci(15));
console.log(performance.now() - start);

start = performance.now();
console.log(fibonacci(15));
console.log(performance.now() - start);
