pub struct EventSubscribers {
    pub subscribers: Vec<(String, Vec<(usize, String)>)>
}

impl EventSubscribers {
    pub fn get_subscribers_for_event(&self, event: &str) -> Vec<(usize, String)> {
        let mut search_index = 0;
        loop {
            if self.subscribers[search_index].0 == event {
                return self.subscribers[search_index].1.clone();
            }
            search_index += 1;
        }
    }

    pub fn add_subscriber_for_event(&mut self, event: &str, subscriber: (usize, String)) {
        let mut search_index = 0;
        loop {
            if self.subscribers[search_index].0 == event {
                self.subscribers[search_index].1.push(subscriber.clone());
                break;
            }
            search_index += 1;
        }
    }
}