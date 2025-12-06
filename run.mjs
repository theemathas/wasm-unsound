import { call_me, initialize } from './pkg/wasm_unsound.js';

initialize();

let i = 0;
while (true) {
    i += 1;
    if (i % 1000000 == 0) {
        // We're done after this prints 2147000000
        console.log(i);
    }
    try {
        console.log(call_me());
        break;
    } catch (e) {
        // Do nothing
    }
}