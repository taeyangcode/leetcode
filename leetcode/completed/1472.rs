struct BrowserHistory {
    history: std::vec::Vec<String>,
    index: i32,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        return BrowserHistory {
            history: vec![homepage],
            index: 0,
        };
    }
    
    fn visit(&mut self, url: String) {
        self.history.truncate(self.index + 1);
        self.history.push(url);
        self.index += 1;
    }
    
    fn back(&mut self, steps: i32) -> String {
        if self.index - steps < 0 {
            self.index = 0;
            return self.history.get(0).unwrap().to_string();
        }
        self.index -= steps;
        return self.history.get(self.index as usize).unwrap().to_string();
    }
    
    fn forward(&mut self, steps: i32) -> String {
        if self.index + steps >= self.history.len() as i32 {
            self.index = self.history.len() as i32 - 1;
            return self.history.get(self.history.len() - 1).unwrap().to_string();
        }
        self.index += steps;
        return self.history.get(self.index as usize).unwrap().to_string();
    }
}

