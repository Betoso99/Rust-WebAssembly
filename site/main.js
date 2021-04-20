import * as wasm from "rust_webassembly"

document.getElementById('btn').addEventListener('click', function(){
    const valor1 = document.getElementById('valor1').value;
    const valor2 = document.getElementById('valor2').value;
    wasm.add(valor1, valor2);

});