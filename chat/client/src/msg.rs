use std::{collections::HashMap};
use rbtree::RBTree;
use chrono::{DateTime, Utc};
use chrono_tz::{Asia::Seoul, Tz};
use lamport::Lamport;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Msg{
    id : String, 
    lamport_clock : Lamport, 
    msg: String,
    real_clock : DateTime<Tz>,
}

impl Msg{
    pub fn new(id : String, lamport_clock : Lamport, msg : String) -> Self{
        Msg{
            id,
            lamport_clock,
            msg,
            real_clock :Utc::now().with_timezone(&Seoul),
        }
    }

    pub fn get_id(&self) -> String{
        self.id.clone()
    }

    pub fn get_lamport_clock(&self) -> Lamport{
        self.lamport_clock.clone()
    }

    pub fn get_msg(&self) -> String{
        self.msg.clone()
    }

    pub fn get_real_clock(&self) ->  DateTime<Tz>{
        self.real_clock
    }
} 


#[derive(Debug, Clone)]
pub struct MsgStore{
    rb : RBTree<Lamport, Msg>,
    latest_vv : HashMap<String, u32>, 
    local_lamport : Lamport,
}

impl MsgStore{
    pub fn new(local_id : String) -> Self{
        let mut hash = HashMap::new();

        hash.insert( local_id.clone() , 0);

        MsgStore{
            rb : RBTree::new(),
            latest_vv : hash,
            local_lamport : Lamport::new(local_id),
        }
    }

    pub fn insert(&mut self, msg : String){
        // update clock
        self.local_lamport.tick();

        let new_msg = Msg::new(
            self.local_lamport.get_id(),
             self.local_lamport.clone(), msg);
        
        self.rb.insert(self.local_lamport.clone(), new_msg.clone());
    }

    // n = 0 <- show all
    pub fn print(&self, n : i32){
        print!("\x1B[2J\x1B[1;1H");

        let limit: i32;
        if n == 0 {
            limit = 0;
        }
        else {
            limit = self.rb.len() as i32 - n;
        }

        let mut cnt = 0;

        for (_, v) in self.rb.iter(){

            if cnt < limit {
                cnt += 1 ;
                continue;
            }

            let c = self.latest_vv.get(&v.get_id()).unwrap();
            if *c >= v.get_lamport_clock().get_clock() {
                println!("{} {} \n - {}",
                v.get_id().green(),
                v.get_real_clock().format("%Y/%m/%d %H:%M"),
                v.get_msg().bold());
            }
            else {
                println!("{} {} \n - {}",
                v.get_id().yellow(),
                v.get_real_clock().format("%Y/%m/%d %H:%M"),
                v.get_msg().italic());
            }

            
        }

        if n > self.rb.len() as i32{
            for _ in 0..n-self.rb.len() as i32 {
                println!("\n");
            }
        }   

    }
}