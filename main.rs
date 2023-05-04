//import necessary crates
use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet};
use rand::Rng;
use std::fs::File;

// Define the data structure to represent Instagram accounts
#[derive(Debug, Deserialize)]
struct InstagramAccount {
    username: String,
    country: String,
    followers: u32,
    comments: u32,
}

// Load the dataset  into a vector of Instagram accounts
fn load_data() -> Vec<InstagramAccount> {
    let file = std::fs::File::open("instagram_global_top_1000.csv").unwrap();
    let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(file);
    reader.deserialize().map(|result| result.unwrap()).collect()
}

// Create a graph to represent the relationships between Instagram accounts
fn create_graph(instagram_accounts: &[InstagramAccount]) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    for account in instagram_accounts {
        for other_account in instagram_accounts {
            if account.country == other_account.country && account.username != other_account.username {
                graph.entry(account.username.clone()).or_default().insert(other_account.username.clone());
            }
        }
    }
    graph
}

// Use BFS to find the average distance between pairs of vertices in the graph
fn bfs_distance(graph: &HashMap<String, HashSet<String>>, start: &str) -> f32 {
    let mut visited = HashSet::new();
    let mut queue = vec![start.to_string()];
    let mut distance = 0;
    let mut level_size = 1;
    let mut next_level_size = 0;
    let mut total_distance = 0;
    visited.insert(start);
    while !queue.is_empty() {
        let current = queue.remove(0);
        level_size -= 1;
        for neighbor in graph.get(&current).unwrap().iter() {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push(neighbor.clone());
                next_level_size += 1;
                total_distance += distance + 1;
            }
        }
        if level_size == 0 {
            distance += 1;
            level_size = next_level_size;
            next_level_size = 0;
        }
    }
    if visited.len() == 1 {
        0.0 // if the start node is the only node in the graph, the distance is 0
    } else {
        total_distance as f32 / ((visited.len() - 1) * visited.len()) as f32
    }
}


// Determine whether users in the same audience country tend to have similar follower counts
fn main() {
    let instagram_accounts = load_data();
    let graph = create_graph(&instagram_accounts);
    let mut country_followers = HashMap::new();

    for account in instagram_accounts {
        let followers_sum = country_followers.entry(account.country
        ).or_insert(0);
        *followers_sum += account.followers;

            }

        let mut country_average_distances = HashMap::new();

        for (country, followers_sum) in country_followers.iter() {
        let mut accounts_in_country = vec![];
        for account in instagram_accounts.iter().filter(|a| a.country == *country) {
        accounts_in_country.push(account.username.clone());
            }
        let mut distances = vec![];
        for start in accounts_in_country.iter() {
        let distance_sum = accounts_in_country.iter()
        .filter(|a| a != &start)
        .map(|a| bfs_distance(&graph, a))
        .sum::<f32>();
        distances.push(distance_sum / (accounts_in_country.len() - 1) as f32);
            }

        let average_distance = distances.iter().sum::<f32>() / distances.len() as f32;
        let average_followers = *followers_sum as f32 / accounts_in_country.len() as f32;
        country_average_distances.insert(country, (average_distance, average_followers));
            } 
        // for each country, calculates the average of the distances and the average number of followers,
        for (country, (average_distance, average_followers)) in country_average_distances.iter() {
        println!("{}: Average distance = {}, Average followers = {}", country, average_distance, average_followers);
            }
                }

        
        #[cfg(test)]
        mod tests {
        use super::*;
        #[test]

fn test_load_data() {
    let data = load_data();
    assert_eq!(data.len(), 1000);
    assert_eq!(data[0].username, "instagram");
    assert_eq!(data[0].country, "United States");
    assert_eq!(data[0].followers, 308000000);
    assert_eq!(data[0].comments, 0);
}

#[test]
fn test_create_graph() {
    let data = vec![
        InstagramAccount { username: "a".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "b".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "c".to_string(), country: "Canada".to_string(), followers: 1, comments: 0 },
    ];
    let graph = create_graph(&data);
    assert!(graph.contains_key("a"));
    assert!(graph.contains_key("b"));
    assert!(graph.contains_key("c"));
    assert!(graph.get("a").unwrap().contains("b"));
    assert!(graph.get("b").unwrap().contains("a"));
    assert!(!graph.get("a").unwrap().contains("c"));
    assert!(!graph.get("b").unwrap().contains("c"));
}

#[test]
fn test_bfs_distance() {
    let data = vec![
        InstagramAccount { username: "a".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "b".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "c".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "d".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "e".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "f".to_string(), country: "United States".to_string(),
        followers: 1, comments: 0 },
        InstagramAccount { username: "g".to_string(), country: "Canada".to_string(), followers: 1, comments: 0 },
        ];
        let graph = create_graph(&data);
        let distance = bfs_distance(&graph, "a", "c");
        assert_eq!(distance, None);
        let distance = bfs_distance(&graph, "a", "b");
        assert_eq!(distance, Some(1));
        let distance = bfs_distance(&graph, "a", "f");
        assert_eq!(distance, Some(2));
        let distance = bfs_distance(&graph, "a", "g");
        assert_eq!(distance, None);
        }
        
        #[test]
        fn test_country_followers() {
        let data = vec![
        InstagramAccount { username: "a".to_string(), country: "United States".to_string(), followers: 1, comments: 0 },
        InstagramAccount { username: "b".to_string(), country: "United States".to_string(), followers: 2, comments: 0 },
        InstagramAccount { username: "c".to_string(), country: "Canada".to_string(), followers: 3, comments: 0 },
        InstagramAccount { username: "d".to_string(), country: "Canada".to_string(), followers: 4, comments: 0 },
        InstagramAccount { username: "e".to_string(), country: "United States".to_string(), followers: 5, comments: 0 },
        InstagramAccount { username: "f".to_string(), country: "United States".to_string(), followers: 6, comments: 0 },
        InstagramAccount { username: "g".to_string(), country: "Canada".to_string(), followers: 7, comments: 0 },
        ];
        let country_followers = get_country_followers(&data);
        assert_eq!(country_followers.get("United States"), Some(&14));
        assert_eq!(country_followers.get("Canada"), Some(&14));
        assert_eq!(country_followers.get("Brazil"), None);
        } 
    }

        
        