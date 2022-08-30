let hello;

function loadWebAssembly(fileName) {
  return fetch(fileName)
    .then(response => response.arrayBuffer())
    .then(buffer => WebAssembly.compile(buffer))
    .then(module => {return new WebAssembly.Instance(module) });
};
  
loadWebAssembly('main.wasm')
  .then(instance => {
    hello = instance.exports._Z5hellov;
    console.log('done compiling!');
  }); 
