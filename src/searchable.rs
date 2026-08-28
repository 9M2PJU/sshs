use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub trait SearchableItem {
    fn search_texts(&self) -> Vec<&str>;
}

pub struct Searchable<T>
where
    T: Clone + SearchableItem,
{
    sort_by_score: bool,
    vec: Vec<T>,
    matcher: SkimMatcherV2,
    filtered: Vec<T>,
}

impl<T> Searchable<T>
where
    T: Clone + SearchableItem,
{
    #[must_use]
    pub fn new(sort_by_score: bool, vec: Vec<T>, search_value: &str) -> Self {
        let mut searchable = Self {
            sort_by_score,
            vec,
            matcher: SkimMatcherV2::default(),
            filtered: Vec::new(),
        };
        searchable.search(search_value);
        searchable
    }

    pub fn search(&mut self, value: &str) {
        if value.is_empty() {
            self.filtered.clone_from(&self.vec);
            return;
        }

        let mut items: Vec<_> = self
            .vec
            .iter()
            .filter_map(|item| {
                let score = item
                    .search_texts()
                    .iter()
                    .filter_map(|text| self.matcher.fuzzy_match(text, value))
                    .max()?;

                Some((item.clone(), score))
            })
            .collect();

        if self.sort_by_score {
            items.sort_by_key(|item| std::cmp::Reverse(item.1));
        }

        self.filtered = items.into_iter().map(|(item, _)| item).collect();
    }

    #[allow(clippy::must_use_candidate)]
    pub fn len(&self) -> usize {
        self.filtered.len()
    }

    #[allow(clippy::must_use_candidate)]
    pub fn is_empty(&self) -> bool {
        self.filtered.is_empty()
    }

    pub fn non_filtered_iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.filtered.iter()
    }
}

impl<'a, T> IntoIterator for &'a Searchable<T>
where
    T: Clone + SearchableItem,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.filtered.iter()
    }
}

impl<T> std::ops::Index<usize> for Searchable<T>
where
    T: Clone + SearchableItem,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.filtered[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Item(&'static str, &'static str);

    impl SearchableItem for Item {
        fn search_texts(&self) -> Vec<&str> {
            vec![self.0, self.1]
        }
    }

    #[test]
    fn test_search_matches_every_text_of_an_item() {
        let items = vec![
            Item("alpha", "one.example.com"),
            Item("beta", "two.example.com"),
        ];
        let mut searchable = Searchable::new(false, items, "");
        assert_eq!(searchable.len(), 2);

        searchable.search("two");
        assert_eq!(searchable.len(), 1);
        assert_eq!(searchable[0].0, "beta");
    }
}
