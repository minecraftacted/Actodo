pub struct Step {
    pub name: String,
    is_complete: bool,
}
impl Step {
    pub fn new(name:String) -> Step{
        Step {name:name,is_complete:false}
    }
    pub fn complete(&mut self) {
        self.is_complete=true;
    }
    pub fn incomplete(&mut self) {
        self.is_complete=false;
    }
}