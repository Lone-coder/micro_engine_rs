
pub trait Translateable{
    fn translate(&mut self, val:(i32,i32));
}

// idk if Self will work
pub trait Interactable{
    fn interact(&mut self,val:&mut Self);
}
