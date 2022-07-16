use tui::widgets::TableState;

pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            state: TableState::default(),
            items,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.state.select(None);
    }

    pub fn selected(&self) -> Option<&T> {
        self.items.get(self.state.selected()?)
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(selected) => {
                if selected >= self.items.len() - 1 {
                    0
                } else {
                    selected + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(selected) => {
                if selected == 0 {
                    self.items.len() - 1
                } else {
                    selected - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], None)]
    #[case(vec!["a", "b", "c"], None)]
    pub fn test_new(#[case] items: Vec<&str>, #[case] selected: Option<usize>) {
        let stateful_table = StatefulTable::<&str>::new(items.clone());
        assert_eq!(stateful_table.state.selected(), selected);
        assert_eq!(stateful_table.items, items);
    }

    #[test]
    pub fn test_set_items() {
        let mut stateful_table = StatefulTable::<&str>::new(vec!["a"]);
        stateful_table.state.select(Some(0));
        let new_items = vec!["a", "b"];
        stateful_table.set_items(new_items.clone());
        assert_eq!(stateful_table.selected(), None);
        assert_eq!(stateful_table.items, new_items);
    }

    #[rstest]
    #[case(None, None)]
    #[case(Some(0), Some(&"a"))]
    #[case(Some(1), Some(&"b"))]
    #[case(Some(2), Some(&"c"))]
    pub fn test_selected(
        #[case] selected_index: Option<usize>,
        #[case] selected_value: Option<&&str>,
    ) {
        let items = vec!["a", "b", "c"];
        let mut stateful_table = StatefulTable::<&str>::new(items);
        stateful_table.state.select(selected_index);
        assert_eq!(stateful_table.selected(), selected_value);
    }

    #[rstest]
    #[case(None, Some(0))]
    #[case(Some(0), Some(1))]
    #[case(Some(1), Some(2))]
    #[case(Some(2), Some(0))]
    pub fn test_next(
        #[case] selected_before: Option<usize>,
        #[case] selected_after: Option<usize>,
    ) {
        let items = vec!["a", "b", "c"];
        let mut stateful_table = StatefulTable::<&str>::new(items);
        stateful_table.state.select(selected_before);
        stateful_table.next();
        assert_eq!(stateful_table.state.selected(), selected_after);
    }

    #[rstest]
    #[case(None, Some(0))]
    #[case(Some(0), Some(2))]
    #[case(Some(1), Some(0))]
    #[case(Some(2), Some(1))]
    pub fn test_previous(
        #[case] selected_before: Option<usize>,
        #[case] selected_after: Option<usize>,
    ) {
        let items = vec!["a", "b", "c"];
        let mut stateful_table = StatefulTable::<&str>::new(items);
        stateful_table.state.select(selected_before);
        stateful_table.previous();
        assert_eq!(stateful_table.state.selected(), selected_after);
    }
}
