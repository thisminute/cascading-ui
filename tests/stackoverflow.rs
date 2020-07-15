extern crate cwf;
use cwf::{ cwf_lib, cwf_dom };

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

cwf_lib!();

#[wasm_bindgen_test]
fn compile() {
   cwf_dom! {
      title: "Stack Overflow";

      header {
         hamburger {}
         logo {}
         products {}
         search {}
         icons {
            profile {}
            inbox {}
            achievements {}
            review {}
            help {}
            // https://github.com/thisminute/cascading-wasm-framework/issues/2
            // site-switcher {}
         }
      }
      content {
         mainbar {
            headline {}
            filter {}
            list {}
         }
         sidebar {
            stuff {}
         }
      }
   }
}
