pub(crate) struct DoveCote {
    boxes: Vec<usize>,
    total_doves: usize,
    pub first_collision: usize,
    pub all_one: usize,
    pub all_two: usize,
    pub empty_boxes: usize,
    pub max_doves: usize,
    // pub empty_boxes_fn: RevFn
}

impl DoveCote {
    pub fn new(size: usize) -> Self {
        let boxes = vec![0; size];
        // let mut empty_boxes_fn = RevFn{ function: vec![0; size]};
        return DoveCote {
            boxes,
            total_doves: 0,
            first_collision: 0,
            empty_boxes: size,
            all_one: 0,
            all_two: 0,
            max_doves: 0,
            // empty_boxes_fn,
        };
    }

    pub fn throw(&mut self, box_num: usize) -> bool {
        self.boxes[box_num] += 1;
        self.total_doves += 1;
        let curr_count = self.boxes[box_num];

        if curr_count == 1 {
            if self.total_doves < self.boxes.len() {
                self.empty_boxes -= 1;
            }
            self.all_one += 1;
            // self.empty_boxes_fn.set(self.total_doves, self.empty_boxes);
        } else if curr_count == 2 {
            self.all_two += 1;
            if self.first_collision == 0 {
                self.first_collision = self.total_doves;
            }
        }

        if self.total_doves <= self.boxes.len() {
            if curr_count > self.max_doves {
                self.max_doves = curr_count;
            }
        } else {
            // return false;
        }

        if self.all_one == self.boxes.len() {
            self.all_one = self.total_doves;
        } else if self.all_two == self.boxes.len() {
            self.all_two = self.total_doves;
            return false;
        }
        return true;
    }

    // pub fn wrap_up(&mut self) {
    //     self.all_two = self.total_doves;
    // }

    pub fn clone(&self) -> Self {
        return DoveCote {
            boxes: self.boxes.clone(),
            total_doves: self.total_doves,
            first_collision: self.first_collision,
            empty_boxes: self.empty_boxes,
            all_one: self.all_one,
            all_two: self.all_two,
            max_doves: self.max_doves,
            // empty_boxes_fn: self.empty_boxes_fn.clone(),
        };
    }

    pub fn get_hole(&self, index: usize) -> usize {
        return self.boxes[index];
    }
}