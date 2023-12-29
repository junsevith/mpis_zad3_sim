pub(crate) struct DoveCote {
    boxes: Vec<usize>,
    total_doves: usize,
    pub first_collision: usize,
    pub all_one: usize,
    pub all_two: usize,
    pub empty_boxes: usize,
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
            // empty_boxes_fn,
        };
    }

    pub fn throw(&mut self, box_num: usize) {
        self.boxes[box_num] += 1;
        self.total_doves += 1;
        let curr_count = self.boxes[box_num];

        if curr_count == 1 {
            if self.total_doves < self.boxes.len() {
                self.empty_boxes -= 1;
            }
            // self.empty_boxes_fn.set(self.total_doves, self.empty_boxes);
        } else if curr_count == 2 {
            self.all_one += 1;
            if self.first_collision == 0 {
                self.first_collision = self.total_doves;
            }
        } else if curr_count == 3 {
            self.all_two += 1;
        }

        if self.all_one == self.boxes.len(){
            self.all_one = self.total_doves;
        }
    }

    pub fn wrap_up(&mut self) {
        self.all_two = self.total_doves;
    }
}