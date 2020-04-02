pub trait Messagable {
    fn send(&self);
}

pub enum SendValue{
    Number{
        index:usize,
        val:Vec<f32>
    },
    Request{
        val:usize
    },
    EntityLoad{
        val:Box<SendValue>
    }
}
