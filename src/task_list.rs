use crate::task::Task;
struct TaskList {
    tasks:Vec<Task>,
}
impl TaskList {
    fn new() ->TaskList {
        TaskList {tasks: Vec::new()}
    }
    fn add_task(&mut self,name:String) {
        let new_task = Task::new(name);
        self.tasks.push(new_task);
    }
    fn remove_task(&mut self,index:usize) ->Result<(),()> {
        if(index>=self.tasks.len()) {
            return Err(())
        }
        self.tasks.remove(index);
        Ok(())
    }
    fn complete_task(&mut self,index:usize) -> Result<(),()> {
        let task = self.tasks.get_mut(index);
        if task.is_none() {
            return Err(())
        }
        task.unwrap().complete();
        Ok(())
    }
    fn incomplete_task(&mut self,index:usize) -> Result<(),()> {
        let task = self.tasks.get_mut(index);
        if task.is_none() {
            return Err(())
        }
        task.unwrap().incomplete();
        Ok(())
    }
    fn add_step(&mut self,index:usize,name:String) -> Result<(), ()>{
        let task = self.tasks.get_mut(index);
        if task.is_none() {
            return Err(())
        }
        task.unwrap().add_step(name);
        Ok(())
    }
    fn remove_step(&mut self,tasks_index:usize,steps_index:usize) ->Result<(),()> {
        let task = self.tasks.get_mut(tasks_index);
        if task.is_none() {
            return Err(())
        }
        task.unwrap().remove_step(steps_index)
    }
    fn complete_step(&mut self,tasks_index:usize,steps_index:usize) -> Result<(),()> {
        let task = self.tasks.get_mut(tasks_index);
        if task.is_none() {
            return Err(())
        }
        task.unwrap().complete_step(steps_index)
    }
    fn incomplete_step(&mut self,tasks_index:usize,steps_index:usize) -> Result<(),()> {
        let task = self.tasks.get_mut(tasks_index);
        if task.is_none() {
            return Err(())
        }
        task.unwrap().complete_step(steps_index)
    }
}