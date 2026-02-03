use std :: collection :: {HashMap,HashSet};

pub struct DependencyGraph{

    //module -> dependencies

    pub map : HashMap <String, Vec<String>>,
}

impl DependencyGraph{
    pub fn new() -> Self {
        DependencyGraph{
            map:HashMap :: new(),
        }
    }

    pub fn add_module(&mut self , name :&str ,deps : Vec<String>){
        self.map.insert(name.to_string(),deps);
    }

    pub fn print(&self){
        println!("Dependency tree : ");
        if self.map.is_empty(){
            println!("empty");
            return;
        }

        for (module,deps) in &self.map {
            if deps.is_empty(){
                println!("module -> no dependencies");
            }
            else {
                println!("{module} -> {:?}", deps);
            }
            
        }


        pub fn remove_module (&mut self ,target :&str){

            let mut remove_list = Vec::new();

            for (module,deps) in &self.map{
                if deps.contains(&target.to_string()){
                    remove_list.push(module.clone());
                }
            }
            remove_list.push(target.to_string());

            println!("\nRemoving modules : ");

            for m in &remove_list{
                println!("- {}",m);
                self.map.remove(m)
            }

            for deps in self.map.values_mut(){
                deps.retain(|d| d!=target );
            }
        }
    }
}