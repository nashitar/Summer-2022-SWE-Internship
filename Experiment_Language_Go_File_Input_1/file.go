package main
import (
	"fmt" // implements formatted I/O with functions analogous to C's printf and scanf
	"syscall/js" // gives access to the WebAssembly host environment when using the js/wasm architecture
)
func main() {
	c := make(chan struct{}, 0) // open channel to browser
	document := js.Global().Get("document") // access the html document
	input := document.Call("getElementById", "inputFile") // get file input
	input.Set("oninput", js.FuncOf(func(this js.Value, i []js.Value) interface{} { // when a file is inputted
		input.Get("files").Call("item", 0).Call("arrayBuffer").Call("then", js.FuncOf(func(this js.Value, i []js.Value) interface{} {
			data := js.Global().Get("Uint8Array").New(i[0]) // get file data into an array buffer
			destination := make([]byte, data.Get("length").Int()) // make a byte array in go the same length as the js array buffer
			js.CopyBytesToGo(destination, data) // copy values from js array buffer to golang byte array
			output := string(destination) // convert byte array to string
			fmt.Println(output) // write to console
			document.Call("getElementById", "outputFile").Set("innerText", output) // write to page
			return nil // exit from function that opened file to an array buffer
		}))
		return nil // exit from function that accessed the input element
	}))
	<- c // close channel (no mem leaks)
}
