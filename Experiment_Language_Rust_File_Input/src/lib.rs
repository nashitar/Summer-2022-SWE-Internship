use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // allows user to cast javascript values between different types
use js_sys:: JsString; // string type in javascript
use web_sys:: {Event, FileReader, HtmlInputElement}; // gets these three elements from JS

#[wasm_bindgen(start)]
pub fn main() {
    // instantiate object to connect to browser console log through wasm
    wasm_logger::init(wasm_logger::Config::default()); 

    let window = web_sys::window().unwrap(); // browser window object
    let document = window.document(); // html document object
    
    // instantiate file reader
    let reader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();

    // rust closure: temporary function that can be stored as a variable or 
    // passed as a function parameter
    let onload = Closure::wrap(Box::new(move|event: Event| {
        // get the filereader element
        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        
        // get file contents 
        let data = element.result().unwrap();
        
        // convert file contents to string
        let filestring: JsString = data.dyn_into::<JsString>().unwrap();
        
        // send file string to console log
        log::info!("{}", filestring); 
    }) as Box<dyn FnMut(_)>);
    
    // when the file reader is called on a file run the onload closure
    reader.set_onloadend(Some(onload.as_ref().unchecked_ref())); 
    
    // leaks the closure so that it is valid for the entire program 
    // WARNING: leaks memory
    onload.forget();

    // as_ref is so that it makes a copy rather than directly modify the document variable
    // confirming that there is a document, define an input element
    let input: HtmlInputElement = document.as_ref().expect("must have document")
                                        .create_element("input").unwrap()
                                        .dyn_into::<HtmlInputElement>().unwrap();
    
    // the input will be a file
    input.set_type("file");
    
    // add the input element to the document
    document.unwrap().body().expect("must have a body").append_child(&input).unwrap();
    
    // rust closure: temporary function that can be stored as a variable or 
    // passed as a function paramter
    let callback = Closure::wrap(Box::new(move|event: Event| {
        // get the input element
        let element = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        
        // get the contents of the input element
        let list = element.files().unwrap();

        // contents appear as a list so select the first file and confirm 
        // that it is valid
        let _file = list.get(0).expect("must have a file handle");
        
        // use the file reader to read in that file
        reader.read_as_binary_string(&_file).unwrap();
    }) as Box<dyn FnMut(_)>);
    
    // on "change" get what was read from the callback 
    input.add_event_listener_with_callback("change", 
                                           callback.as_ref().unchecked_ref()).unwrap();
    
    // leaks the closure so that it is valid for the entire program 
    // WARNING: leaks memory
    callback.forget();
}
