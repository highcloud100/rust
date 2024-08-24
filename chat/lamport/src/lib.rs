use std::cmp::Ordering;

#[derive(Debug, Clone, Default, Eq)]
pub struct Lamport{
    clock : u32,
    id : String,
}

impl Lamport{
    pub fn new(id : String) -> Self{
        Lamport{
            clock : 0,
            id,
        }
    }

    pub fn tick(&mut self){
        self.clock += 1;
    }

    pub fn update(&mut self, other : &Lamport){
        self.clock = self.clock.max(other.clock) + 1;
    }

    pub fn get_clock(&self) -> u32{
        self.clock
    }

    pub fn get_id(&self) -> String{
        self.id.clone()
    }
}

impl Ord for Lamport {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.clock.cmp(&other.clock) == Ordering::Equal {
            self.id.cmp(&other.id)
        } else {
            self.clock.cmp(&other.clock)
        }
    }
}

impl PartialOrd for Lamport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Lamport {
    fn eq(&self, other: &Self) -> bool {
        self.clock == other.clock && self.id == other.id
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_lamport() {
        let mut lamport = Lamport::new("1".to_string());
        assert_eq!(lamport.clock, 0);
        lamport.tick();
        assert_eq!(lamport.clock, 1);
        lamport.tick();
        assert_eq!(lamport.clock, 2);
    }

    #[test]
    fn test_ordering() {
        let mut lamport1 = Lamport::new("1".to_string());
        let mut lamport2 = Lamport::new("2".to_string());
        assert_eq!(lamport1.cmp(&lamport2), Ordering::Less);
        lamport1.tick();
        assert_eq!(lamport1.cmp(&lamport2), Ordering::Greater);
        lamport2.tick();
        assert_eq!(lamport1.cmp(&lamport2), Ordering::Less);
    }
}