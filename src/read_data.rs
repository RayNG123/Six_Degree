use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::{HashSet, HashMap};

//read a list of edges from a txt file and output a vector of connected edges and a HashSet with all unique node number
pub fn read_graph(directory: &str) -> Result<(Vec<(usize,usize)>,HashSet<usize>), Box<dyn Error>> {
    let file = File::open(directory).expect("Could not open file");
    let reader = BufReader::new(file).lines();
    let mut node_set: HashSet<usize> = HashSet::new();
    let mut edge_list: Vec<(usize,usize)> = Vec::new();
    //read list of edges from a txt file
    for line in reader {
        let line = line.expect("Error reading");
        let parts: Vec<&str> = line.trim().split(' ').collect();

        let node_a = parts[0].parse::<usize>().expect("Error reading");
        let node_b = parts[1].parse::<usize>().expect("Error reading");

        node_set.insert(node_a);
        node_set.insert(node_b);

        edge_list.push((node_a,node_b))    
    }
    Ok((edge_list,node_set))
}

//given a feature number and an ego number of interest, output a Hashmap where key is node number and value is the feature value correspond to that node
pub fn read_feat(directory: usize, feat_num: usize) -> Result<HashMap<usize,usize>, Box<dyn Error>> {
    //find feature number
    let file0 = File::open(format!("facebook/{}.featnames", directory)).expect("Could not open file");
    let reader0 = BufReader::new(file0).lines();
    let mut feature_id = 12345;

    for line in reader0 {
        let line = line.expect("Error reading");
        let parts: Vec<&str> = line.trim().split(' ').collect();

        let index = parts[3].parse::<usize>().expect("Error reading");
      
        if index == feat_num {
            feature_id = parts[0].parse::<usize>().expect("Error reading") + 1;
            break;
        }
    }

    //if feature not find, stop the process
    if feature_id == 12345 {
        return Err(format!("directory {} feature id not found", directory).into());
    }

    //read the feature
    let file1 = File::open(format!("facebook/{}.feat", directory)).expect("Could not open file");
    let reader1 = BufReader::new(file1).lines();
    let mut feat = HashMap::new();
    
    for line in reader1 {
        let line = line.expect("Error reading");
        let parts: Vec<&str> = line.trim().split(' ').collect();
        let node_a = parts[feature_id].parse::<usize>().expect("Error reading");
        let node_b = parts[0].parse::<usize>().expect("Error reading");
        feat.insert(node_b,node_a);
    }
    //read the feature for the ego node
    let file2 = File::open(format!("facebook/{}.egofeat", directory)).expect("Could not open file");
    let mut reader2 = BufReader::new(file2).lines();
    let egoline = reader2.next().unwrap()?;
    let parts: Vec<&str> = egoline.trim().split(' ').collect();
    let node_a = parts[feature_id].parse::<usize>().expect("Error reading");
    feat.insert(directory,node_a);
    Ok(feat)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_feat () {
        let feat1 = read_feat(107,4).unwrap();
        assert_eq!(*feat1.get(&896).unwrap(), 1);
        assert_eq!(*feat1.get(&897).unwrap(), 0);
        assert_eq!(*feat1.get(&902).unwrap(), 1);
        assert_eq!(*feat1.get(&898).unwrap(), 0);
    }
}