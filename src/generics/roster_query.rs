// generics/roster_query.rs
//
// generic stat queries over the 2004 patriots roster.
// `top_n` works on any orderable stat; `StatLeader` holds a typed winner.

use std::fmt;

/// generic stat leader -- holds a player name and their typed stat value.
/// `T` can be yards (u32), rating (f32), touchdowns (u8), etc.
#[derive(Debug, Clone)]
pub struct StatLeader<T: fmt::Display + Clone> {
    pub name: String,
    pub stat_name: String,
    pub value: T,
}

impl<T: fmt::Display + Clone> StatLeader<T> {
    pub fn new(name: &str, stat_name: &str, value: T) -> Self {
        Self {
            name: name.to_string(),
            stat_name: stat_name.to_string(),
            value,
        }
    }
}

impl<T: fmt::Display + Clone> fmt::Display for StatLeader<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} ({})", self.name, self.value, self.stat_name)
    }
}

/// a player stat entry pairing a name with a comparable value
#[derive(Debug, Clone)]
pub struct PlayerStat<T> {
    pub name: String,
    pub value: T,
}

impl<T> PlayerStat<T> {
    pub fn new(name: &str, value: T) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

/// returns the top N items from a slice, sorted descending by value.
/// works on any type implementing Ord (integers, strings, etc).
///
/// monomorphization: the compiler generates one version of this function
/// for each concrete type it's called with (u32, u16, etc).
pub fn top_n<T: Ord + Clone>(items: &[PlayerStat<T>], n: usize) -> Vec<&PlayerStat<T>> {
    let mut sorted: Vec<&PlayerStat<T>> = items.iter().collect();
    sorted.sort_by(|a, b| b.value.cmp(&a.value));
    sorted.truncate(n);
    sorted
}

/// same as top_n but for floating-point stats (f32 is not Ord).
/// uses PartialOrd with a total_cmp fallback for NaN safety.
pub fn top_n_float(items: &[PlayerStat<f32>], n: usize) -> Vec<&PlayerStat<f32>> {
    let mut sorted: Vec<&PlayerStat<f32>> = items.iter().collect();
    sorted.sort_by(|a, b| b.value.total_cmp(&a.value));
    sorted.truncate(n);
    sorted
}

/// find the leader in a stat category.
/// trait bound: T must be PartialOrd (for comparison) and Display + Clone (for StatLeader).
pub fn find_leader<T>(items: &[PlayerStat<T>], stat_name: &str) -> Option<StatLeader<T>>
where
    T: PartialOrd + fmt::Display + Clone,
{
    items
        .iter()
        .max_by(|a, b| {
            a.value
                .partial_cmp(&b.value)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|ps| StatLeader::new(&ps.name, stat_name, ps.value.clone()))
}

/// generic filter: keep only stats above a threshold.
/// demonstrates where clause with multiple bounds.
pub fn above_threshold<'a, T>(items: &'a [PlayerStat<T>], threshold: &T) -> Vec<&'a PlayerStat<T>>
where
    T: PartialOrd,
{
    items.iter().filter(|ps| ps.value > *threshold).collect()
}

/// sum all values in a stat slice.
/// demonstrates trait bounds with std::iter::Sum.
pub fn total_stat<T>(items: &[PlayerStat<T>]) -> T
where
    T: std::iter::Sum + Clone,
{
    items.iter().map(|ps| ps.value.clone()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rushing_yards() -> Vec<PlayerStat<u32>> {
        vec![
            PlayerStat::new("Corey Dillon", 1635),
            PlayerStat::new("Kevin Faulk", 255),
            PlayerStat::new("Patrick Pass", 141),
            PlayerStat::new("Tom Brady", 28),
            PlayerStat::new("Cedric Cobbs", 90),
        ]
    }

    fn passer_ratings() -> Vec<PlayerStat<f32>> {
        vec![
            PlayerStat::new("Tom Brady", 92.6),
            PlayerStat::new("Rohan Davey", 0.0),
        ]
    }

    fn receiving_tds() -> Vec<PlayerStat<u8>> {
        vec![
            PlayerStat::new("David Givens", 3),
            PlayerStat::new("Deion Branch", 4),
            PlayerStat::new("Daniel Graham", 7),
            PlayerStat::new("David Patten", 7),
            PlayerStat::new("Corey Dillon", 1),
        ]
    }

    #[test]
    fn test_top_n_rushing() {
        let yards = rushing_yards();
        let top3 = top_n(&yards, 3);
        assert_eq!(top3.len(), 3);
        assert_eq!(top3[0].name, "Corey Dillon");
        assert_eq!(top3[1].name, "Kevin Faulk");
        assert_eq!(top3[2].name, "Patrick Pass");
    }

    #[test]
    fn test_top_n_touchdowns() {
        let tds = receiving_tds();
        let top2 = top_n(&tds, 2);
        assert_eq!(top2.len(), 2);
        // 7 and 7 -- either graham or patten first (stable sort)
        assert_eq!(top2[0].value, 7u8);
        assert_eq!(top2[1].value, 7u8);
    }

    #[test]
    fn test_top_n_float() {
        let ratings = passer_ratings();
        let top = top_n_float(&ratings, 1);
        assert_eq!(top[0].name, "Tom Brady");
    }

    #[test]
    fn test_find_leader() {
        let yards = rushing_yards();
        let leader = find_leader(&yards, "rushing yards").unwrap();
        assert_eq!(leader.name, "Corey Dillon");
        assert_eq!(leader.value, 1635u32);
        assert_eq!(leader.stat_name, "rushing yards");
    }

    #[test]
    fn test_find_leader_float() {
        let ratings = passer_ratings();
        let leader = find_leader(&ratings, "passer rating").unwrap();
        assert_eq!(leader.name, "Tom Brady");
    }

    #[test]
    fn test_above_threshold() {
        let yards = rushing_yards();
        let over_100 = above_threshold(&yards, &100);
        assert_eq!(over_100.len(), 3); // dillon 1635, faulk 255, pass 141
    }

    #[test]
    fn test_total_stat() {
        let yards = rushing_yards();
        let total: u32 = total_stat(&yards);
        assert_eq!(total, 1635 + 255 + 141 + 28 + 90);
    }

    #[test]
    fn test_stat_leader_display() {
        let leader = StatLeader::new("Corey Dillon", "rushing yards", 1635u32);
        let display = format!("{leader}");
        assert!(display.contains("Corey Dillon"));
        assert!(display.contains("1635"));
        assert!(display.contains("rushing yards"));
    }

    #[test]
    fn test_top_n_more_than_available() {
        let yards = rushing_yards();
        let top10 = top_n(&yards, 10);
        assert_eq!(top10.len(), 5); // only 5 entries
    }

    #[test]
    fn test_empty_slice() {
        let empty: Vec<PlayerStat<u32>> = vec![];
        assert!(top_n(&empty, 3).is_empty());
        assert!(find_leader(&empty, "yards").is_none());
        assert!(above_threshold(&empty, &0).is_empty());
    }
}
