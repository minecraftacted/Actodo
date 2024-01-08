use std::fmt::Error;

use super::step::Step;
pub struct Task {
    pub name: String,
    is_complete: bool,
    steps: Vec<Step>
}
impl Task {
    pub fn new(name: String) -> Task {
        Task{name:name, is_complete:false, steps: Vec::new()}
    }
    pub fn complete(&mut self) {
        self.is_complete = true;
    }
    pub fn incomplete(&mut self) {
        self.is_complete = false;
    }
    pub fn add_step(&mut self,name:String) {
        let new_step = Step::new(name);
        self.steps.push(new_step);
    }
    pub fn remove_step(&mut self,index:usize) ->Result<(),()>{
        if(index>=self.steps.len()) {
            return Err(())
        }
        self.steps.remove(index);
        Ok(())
    }
    pub fn complete_step(&mut self,index:usize) -> Result<(),()>{
        let step = self.steps.get_mut(index);
        if step.is_none() {
            return Err(())
        }
        step.unwrap().complete();
        Ok(())
    }
    pub fn incomplete_step(&mut self,index:usize) -> Result<(),()>{
        let step = self.steps.get_mut(index);
        if step.is_none() {
            return Err(())
        }
        step.unwrap().incomplete();
        Ok(())
    }

}
