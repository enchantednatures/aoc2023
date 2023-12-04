use arangors::document::options::InsertOptions;
use arangors::Connection;
use day_03::extract_gears;
use day_03::extract_numbers;
use day_03::get_positions_touching_number;
use day_03::Gear;
use day_03::GraphRepo;
use day_03::PartNumber;
use futures::future::join_all;
use itertools::Itertools;
use rayon::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Edge {
    #[serde(rename = "_from")]
    from: String,
    #[serde(rename = "_to")]
    to: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bulk {
    documents: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record<T> {
    #[serde(rename = "_key")]
    key: String,

    #[serde(flatten)]
    data: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GearNode {
    pub idx: usize,
    pub position: usize,
}

impl From<&Gear> for GearNode {
    fn from(part: &Gear) -> Self {
        Self {
            idx: part.idx,
            position: part.line,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartNumberNode {
    pub value: usize,
}

impl From<&PartNumber> for PartNumberNode {
    fn from(part: &PartNumber) -> Self {
        Self { value: part.value }
    }
}

#[tokio::main]
async fn main() {
    let conn = Connection::establish_without_auth("http://tower:8529")
        .await
        .unwrap();

    conn.create_gears_connection().await;
    let db = conn.db("_system").await.unwrap();
    let gears_collection = db
        .collection("symbols")
        .await
        .expect("failed to get collection");
    let edge_collection = db
        .collection("gears")
        .await
        .expect("failed to get collection");
    let parts_collection = db
        .collection("parts")
        .await
        .expect("failed to get collection");

    // let edges = Vec::new();
    let input = include_str!("../part1.txt");
    let gears = extract_gears(input).collect_vec();
    let parts = extract_numbers(input).collect_vec();
    // let graph = db
    //     .graph("aoc_2023_day3_part_2")
    //     .await
    //     .expect("failed to get graph");

    let gears_to_create = gears
        .iter()
        .map(|gear| {
            let key = format!("{}-{}", gear.line, gear.idx);
            Record {
                key,
                data: GearNode::from(gear),
            }
        })
        .collect_vec();

    let gears_tasks = gears_to_create.into_iter().map(|gear| {
        gears_collection
            // .create_multiple_documents(gears_to_create, InsertOptions::default())
            .create_document(gear, InsertOptions::default())
    });
    join_all(gears_tasks).await;

    let create_parts_tasks = parts.iter().map(|part| {
        let key = format!("{}-{}-{}", part.line, part.start_index, part.end_index);
        let record: Record<PartNumberNode> = Record {
            key: key.clone(),
            data: PartNumberNode::from(part),
        };
        parts_collection.create_document(record, InsertOptions::default())
    });
    join_all(create_parts_tasks).await;

    let create_edges = parts
        .iter()
        .flat_map(|part| {
            dbg!(&part);
            get_positions_touching_number(part)
                .into_iter()
                .map(|p_edge| {
                    if let Some(gear) = gears
                        .iter()
                        .find(|f| f.line == p_edge.0 && f.line == p_edge.1)
                    {
                        let key = format!("{}-{}-{}", part.line, part.start_index, part.end_index);
                        return Some(edge_collection.create_document(
                            Edge {
                                from: format!("parts/{}", &key.to_string()),
                                to: format!("symbols/{}-{}", gear.line, gear.idx),
                            },
                            InsertOptions::default(),
                        ));
                    }
                    None
                })
        })
        .flatten();

    join_all(create_edges).await;
}
