use csv::Reader;
use std::ops::Range;

#[derive(Debug)]
struct Data {
   headers: Vec<String>,
   data: Vec<Vec<f32>>
}

impl Data {
    fn read_csv(path: &str) -> Data {
        let mut data_vec: Vec<Vec<f32>> = Vec::new();
        let mut headers_vec: Vec<String> = Vec::new();

        let mut reader = Reader::from_path(path).unwrap();

        let headers = reader.headers().unwrap();

        for item in headers{
            headers_vec.push(item.to_string())
        }

        for record in reader.records() {
            let record = record.unwrap();
            let row: Vec<f32> = record.iter().map(|x| x.parse::<f32>().unwrap()).collect();
            data_vec.push(row);
        }

        return Data{headers: headers_vec, data: data_vec}
    }

    fn row(&self, index: usize) -> Vec<f32> {
        self.data[index].clone()
    }

    fn rows(&self, index: Range<usize>) -> Vec<Vec<f32>> {
        let mut rows: Vec<Vec<f32>> = Vec::new();
        for row in index{
            rows.push(self.data[row].clone());
        }
        rows
    }

    fn column(&self, index: usize) -> Vec<f32> {
        let mut column: Vec<f32> = Vec::new();
        for row in &self.data {
            column.push(row[index]);
        }
        column
    }

    fn index(&self, row: usize, column: usize) -> f32 {
        self.data[row][column]
    }

    fn shape(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let data = Data::read_csv("data.csv");
    println!("{:?}", data.shape());
}
