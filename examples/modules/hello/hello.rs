extern crate ogunix;

use self::ogunix::module::Module;

struct Hello {
    /* Just to compile. */
    id:u32   
}

impl Module for Hello {
    
    fn init(&self){

    }
    fn exit(&self){

    }
}
