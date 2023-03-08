pub struct ParkingSystem {
    pub spaces: Vec<i32>
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            spaces: vec![big, medium, small]
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        if self.spaces[(car_type - 1) as usize] > 0 {
            self.spaces[(car_type - 1) as usize] -= 1;
            return true;
        }

        return false;
    }
    
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */


/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
