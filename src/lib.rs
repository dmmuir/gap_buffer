type Text = Vec<char>;

const GAP_SIZE: usize = 1024;
const GAP_PAGE: [char; GAP_SIZE] = ['_'; GAP_SIZE];

#[derive(Debug, Clone)]
pub struct GapBuffer {
    buffer: Text,
    gap_start: usize,
    gap_len: usize,
}

impl GapBuffer {
    pub fn from(data: Text) -> Self {
        let gap_len = GAP_SIZE;
        let buffer = [&GAP_PAGE, data.as_slice()].concat();
        Self {
            buffer,
            gap_start: 0,
            gap_len,
        }
    }

    pub fn move_gap(&mut self, cursor: usize) {
        let mut gap_right = self.gap_start + self.gap_len - 1;

        // If there's no gap, just move the cursor.
        if self.gap_len == 0 {
            self.gap_start = cursor;
            return;
        }

        // Move left
        while cursor < self.gap_start {
            self.gap_start -= 1;
            gap_right -= 1;
            self.buffer[gap_right + 1] = self.buffer[self.gap_start];
            self.buffer[self.gap_start] = '_';
        }

        // Move right
        while cursor > self.gap_start {
            self.gap_start += 1;
            gap_right += 1;
            self.buffer[self.gap_start - 1] = self.buffer[gap_right];
            self.buffer[gap_right] = '_';
        }
    }

    pub fn grow(&mut self, input_size: usize) {
        let number_of_new_pages = input_size / GAP_SIZE + input_size % GAP_SIZE + 1;
        let new_gap_size = GAP_SIZE * number_of_new_pages;
        let mut new_buffer = Vec::with_capacity(self.buffer.len() + new_gap_size);
        new_buffer.push(&self.buffer[..self.gap_start]);

        for _ in 0..number_of_new_pages {
            new_buffer.push(&GAP_PAGE);
        }
        new_buffer.push(&self.buffer[self.gap_start + self.gap_len..]);
        self.buffer = new_buffer.concat();
        self.gap_len = new_gap_size;
    }

    pub fn insert(&mut self, mut input: Text) {
        if input.len() > self.gap_len {
            self.grow(input.len());
        }

        if let Some(gap) = self
            .buffer
            .get_mut(self.gap_start..self.gap_start + self.gap_len)
        {
            if let Some(subgap) = gap.get_mut(..input.len()) {
                subgap.swap_with_slice(&mut input);
                self.gap_start = self.gap_start + input.len();
                self.gap_len -= input.len();
            }
        }
    }

    pub fn delete(&mut self, length: usize) {
        let delete_start = self.gap_start + self.gap_len;
        let delete_end = delete_start + length;
        for i in delete_start..delete_end {
            self.buffer[i] = '_';
        }
        self.gap_len += length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_than_gap_size() {
        println!("Test move_gap on 'Hello World'; a test set smaller than the gap size");
        let data: Vec<char> = "Hello World".chars().collect();
        let data_len = data.len();
        let mut gap_buffer = GapBuffer::from(data);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(5);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(10);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(2);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(data_len);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
    }

    #[test]
    fn larger_than_gap_size() {
        println!("Test move_gap on 'Hello darkness, my old friend...'; a test set larger than the gap size");
        let data: Vec<char> = "Hello darkness, my old friend...".chars().collect();
        let data_len = data.len();
        let mut gap_buffer = GapBuffer::from(data);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(0);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(5);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(10);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(2);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(data_len);
        println!("{}", gap_buffer.buffer.iter().collect::<String>());
    }

    #[test]
    fn moves_inserts_deletes_grows() {
        println!("Test moves, inserts, deletes and grows");
        let data: Vec<char> = "Hello World".chars().collect();
        let mut gap_buffer = GapBuffer::from(data);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(gap_buffer.buffer.len() - gap_buffer.gap_len);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.insert(vec!['!']);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(5);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.insert(vec![',']);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(5);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.insert(" my dearest".chars().collect::<Vec<char>>());
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(18);
        gap_buffer.delete(5);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.insert("Dan".chars().collect::<Vec<char>>());
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.move_gap(gap_buffer.gap_start + 1);
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
        gap_buffer.insert("\nToday's my Birthday!".chars().collect::<Vec<char>>());
        println!("'{}'", gap_buffer.buffer.iter().collect::<String>());
    }
}
